use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest as _, Sha256};
use translator_core::{
    ENV_ALLOW_REMOTE_PROVIDER, ENV_PROVIDER, ENV_PROVIDER_API_KEY_ENV, ENV_PROVIDER_URL,
};

const PROFILE_ID: &str = "bergamot-en-es-linux-x86_64-v1";
const MANIFEST_DIGEST: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

#[test]
fn cli_uses_one_verified_embedded_set_and_preserves_the_source_file() {
    let data_home = installation(
        "success",
        "while IFS= read -r _; do :; done; printf '%s' '{\"wire_version\":1,\"translations\":[\"Texto sintetico publico.\"]}'",
    );
    let workspace = unique_directory("workspace");
    fs::create_dir_all(&workspace).expect("workspace directory");
    let source = workspace.join("source.txt");
    let original = b"Public synthetic text.";
    fs::write(&source, original).expect("source fixture");
    let request = format!(
        r#"{{"file_path":"{}","workspace_root":"{}","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"}}"#,
        source.display(),
        workspace.display()
    );

    let output = run_cli(&data_home, &request);

    assert!(output.status.success(), "CLI failure: {:?}", output.stderr);
    assert_eq!(
        String::from_utf8(output.stdout).expect("UTF-8 stdout"),
        r#"{"translated_text":"Texto sintetico publico."}"#
    );
    assert_eq!(
        fs::read(&source).expect("source after translation"),
        original
    );
}

#[test]
fn explicit_embedded_mode_without_state_fails_without_mock_fallback() {
    let data_home = unique_directory("absent");
    fs::create_dir_all(&data_home).expect("empty data home");

    let output = run_cli(&data_home, &request("Read the docs."));
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 stdout");

    assert!(!output.status.success());
    assert!(stdout.contains("PROVIDER_NOT_CONFIGURED"));
    assert!(!stdout.contains("Lee la documentacion"));
}

#[test]
fn child_stderr_and_paths_are_not_exposed_by_the_cli() {
    let private_marker = "PRIVATE_CHILD_DIAGNOSTIC";
    let data_home = installation(
        "failure",
        &format!("while IFS= read -r _; do :; done; printf '%s' '{private_marker}' >&2; exit 1"),
    );

    let output = run_cli(&data_home, &request("Public synthetic text."));
    let observed = format!(
        "{} {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(!output.status.success());
    assert!(observed.contains("PROVIDER_FAILED"));
    assert!(!observed.contains(private_marker));
    assert!(!observed.contains(data_home.to_string_lossy().as_ref()));
    assert!(!observed.contains("Public synthetic text."));
}

#[test]
fn exclusive_cleanup_lease_blocks_inference_before_spawn() {
    let marker = unique_directory("spawn-marker");
    let data_home = installation(
        "busy",
        &format!(
            "while IFS= read -r _; do :; done; : > '{}'; printf '%s' '{{\"wire_version\":1,\"translations\":[\"unexpected\"]}}'",
            marker.display()
        ),
    );
    let lease_path = data_home.join("zed-en-es-translator/embedded/lease.lock");
    let lease = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(lease_path)
        .expect("lease file");
    fs4::FileExt::try_lock(&lease).expect("exclusive cleanup lease");

    let output = run_cli(&data_home, &request("Public synthetic text."));
    let observed = String::from_utf8(output.stdout).expect("UTF-8 stdout");

    assert!(!output.status.success());
    assert!(observed.contains("PROVIDER_TIMEOUT"));
    assert!(
        !marker.exists(),
        "runner must not spawn while cleanup is active"
    );
}

fn installation(case: &str, runner_body: &str) -> PathBuf {
    let data_home = unique_directory(case);
    let root = data_home.join("zed-en-es-translator/embedded");
    fs::create_dir_all(root.join("sets")).expect("sets directory");
    fs::create_dir_all(root.join("objects")).expect("objects directory");
    fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).expect("private root");
    fs::write(root.join("lifecycle.lock"), b"").expect("state lock file");
    fs::set_permissions(
        root.join("lifecycle.lock"),
        fs::Permissions::from_mode(0o600),
    )
    .expect("state lock permissions");
    fs::write(root.join("lease.lock"), b"").expect("inference lease file");
    fs::set_permissions(root.join("lease.lock"), fs::Permissions::from_mode(0o600))
        .expect("lease permissions");

    let runner = format!("#!/bin/sh\nset -eu\n{runner_body}\n");
    let runner_object = install_object(
        &root,
        "translator-embedded-runtime",
        runner.as_bytes(),
        0o700,
    );
    let model_object = install_object(&root, "model.bergamot", b"controlled model", 0o600);
    let vocabulary_object =
        install_object(&root, "vocabulary.spm", b"controlled vocabulary", 0o600);
    let shortlist_object = install_object(
        &root,
        "lexical-shortlist.bin",
        b"controlled shortlist",
        0o600,
    );

    let set = format!(
        "{{\"schema_version\":1,\"manifest_digest\":\"{MANIFEST_DIGEST}\",\"profile_id\":\"{PROFILE_ID}\",\"runner\":{},\"artifacts\":[{},{},{}],\"verification_state\":\"verified\",\"offline_smoke\":\"passed\",\"resource_gate\":\"passed\",\"license_gate\":\"complete\"}}",
        object_json("runner", &runner_object),
        object_json("model", &model_object),
        object_json("vocabulary", &vocabulary_object),
        object_json("lexical_shortlist", &shortlist_object),
    );
    let set_path = root.join("sets").join(format!("{MANIFEST_DIGEST}.json"));
    fs::write(&set_path, set).expect("installed set");
    fs::set_permissions(&set_path, fs::Permissions::from_mode(0o600)).expect("set permissions");
    let state_path = root.join("state.json");
    fs::write(
        &state_path,
        format!(
            "{{\"schema_version\":1,\"generation\":1,\"profile_id\":\"{PROFILE_ID}\",\"current\":\"{MANIFEST_DIGEST}\",\"previous\":null,\"candidate\":null,\"last_operation\":\"prepare\",\"last_outcome\":\"ready\"}}"
        ),
    )
    .expect("installation state");
    fs::set_permissions(&state_path, fs::Permissions::from_mode(0o600)).expect("state permissions");
    data_home
}

struct InstalledObject {
    digest: String,
    name: String,
    size: usize,
}

fn install_object(root: &Path, name: &str, content: &[u8], mode: u32) -> InstalledObject {
    let digest = sha256(content);
    let directory = root.join("objects").join(&digest);
    fs::create_dir_all(&directory).expect("object directory");
    let path = directory.join(name);
    fs::write(&path, content).expect("object content");
    fs::set_permissions(&path, fs::Permissions::from_mode(mode)).expect("object permissions");
    InstalledObject {
        digest,
        name: name.to_string(),
        size: content.len(),
    }
}

fn object_json(role: &str, object: &InstalledObject) -> String {
    format!(
        "{{\"role\":\"{role}\",\"object_digest\":\"{}\",\"installed_name\":\"{}\",\"installed_size\":{}}}",
        object.digest, object.name, object.size
    )
}

fn sha256(content: &[u8]) -> String {
    Sha256::digest(content)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn request(source_text: &str) -> String {
    format!(
        r#"{{"source_text":"{source_text}","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"}}"#
    )
}

fn run_cli(data_home: &Path, input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_translator-cli"))
        .env(ENV_PROVIDER, "embedded_local")
        .env("XDG_DATA_HOME", data_home)
        .env_remove(ENV_PROVIDER_URL)
        .env_remove(ENV_PROVIDER_API_KEY_ENV)
        .env_remove(ENV_ALLOW_REMOTE_PROVIDER)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn CLI");
    child
        .stdin
        .take()
        .expect("CLI stdin")
        .write_all(input.as_bytes())
        .expect("write request");
    child.wait_with_output().expect("wait for CLI")
}

fn unique_directory(case: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::current_dir()
        .expect("current directory")
        .join("target/embedded-cli-tests")
        .join(format!("{}-{case}-{nanos}", std::process::id()))
}
