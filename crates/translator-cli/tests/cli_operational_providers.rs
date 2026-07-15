use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::Write;
use std::net::TcpListener;
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use translator_core::{
    ENV_ALLOW_REMOTE_PROVIDER, ENV_PROVIDER, ENV_PROVIDER_API_KEY_ENV, ENV_PROVIDER_URL,
    MAX_INPUT_BYTES, PROVIDER_TIMEOUT_MS,
};

mod common;

static OPERATIONAL_PORT: Mutex<()> = Mutex::new(());

#[test]
fn exact_local_profile_translates_through_controlled_loopback() {
    let _port = OPERATIONAL_PORT.lock().expect("operational port lock");
    let url = common::operational_local_response_server(
        r#"{"translatedText":["Salida local controlada."]}"#,
    );

    let output = run_cli(
        request_json("Read the docs.", "text"),
        &[(ENV_PROVIDER, "libretranslate"), (ENV_PROVIDER_URL, &url)],
    );

    assert!(output.status.success(), "CLI failure: {:?}", output.stderr);
}

#[test]
fn local_markdown_translation_preserves_code_and_source_file() {
    let _port = OPERATIONAL_PORT.lock().expect("operational port lock");
    let url = common::operational_local_response_server(
        r#"{"translatedText":["Lee la documentacion."]}"#,
    );
    let workspace = unique_temp_dir("markdown");
    fs::create_dir_all(&workspace).expect("workspace");
    let document = workspace.join("readme.md");
    let original = b"Read the docs.\n\n```rust\nlet x = 1;\n```\n";
    fs::write(&document, original).expect("fixture");
    let request = format!(
        r#"{{"file_path":"{}","workspace_root":"{}","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"markdown"}}"#,
        document.display(),
        workspace.display()
    );

    let output = run_cli(
        request,
        &[(ENV_PROVIDER, "libretranslate"), (ENV_PROVIDER_URL, &url)],
    );

    assert!(output.status.success(), "CLI failure: {:?}", output.stderr);
    assert_eq!(fs::read(&document).expect("source"), original);
    let stdout = String::from_utf8(output.stdout).expect("stdout");
    assert!(stdout.contains("let x = 1;"));
}

#[test]
fn mock_remains_default_without_provider_environment() {
    let output = run_cli(request_json("Read the docs.", "text"), &[]);

    assert_eq!(
        String::from_utf8(output.stdout).expect("stdout"),
        r#"{"translated_text":"Lee la documentacion."}"#
    );
}

#[test]
fn oversized_cli_envelope_fails_before_local_provider_contact() {
    let output = run_cli(
        request_json(&"a".repeat(MAX_INPUT_BYTES + 1), "text"),
        &[
            (ENV_PROVIDER, "libretranslate"),
            (ENV_PROVIDER_URL, "http://127.0.0.1:5000"),
        ],
    );

    assert!(String::from_utf8(output.stdout)
        .expect("stdout")
        .contains("INVALID_INPUT"));
}

#[test]
fn azure_missing_key_fails_before_contact_with_redacted_configuration_error() {
    let output = run_azure_cli(request_json("Read the docs.", "text"), None);

    assert_normalized_failure(
        &output,
        "PROVIDER_NOT_CONFIGURED",
        &["Read the docs.", "AZURE_TRANSLATOR_TEST_KEY"],
    );
}

#[test]
fn every_unconfirmed_azure_process_is_denied_before_contact() {
    for _ in 0..2 {
        let output = run_azure_cli(
            request_json("Read the docs.", "text"),
            Some("fake-public-test-key"),
        );

        assert!(String::from_utf8(output.stdout)
            .expect("stdout")
            .contains("REMOTE_CONFIRMATION_REQUIRED"));
    }
}

#[test]
fn confirmed_synthetic_secret_is_blocked_before_azure_contact() {
    let mut request = request_json("API_KEY=fake_public_test_value_123456", "text");
    request.insert(request.len() - 1, ',');
    request.insert_str(request.len() - 1, r#""remote_confirmed":true"#);
    let output = run_azure_cli(request, Some("fake-public-test-key"));
    let stdout = String::from_utf8(output.stdout).expect("stdout");

    assert!(stdout.contains("SECRET_DETECTED"));
    assert!(!stdout.contains("fake_public_test_value"));
    assert!(!stdout.contains("fake-public-test-key"));
}

#[test]
fn cli_rejects_an_inline_or_unsafe_key_reference_before_contact() {
    let output = run_cli(
        request_json("SOURCE_MARKER_PRIVATE", "text"),
        &[
            (ENV_PROVIDER, "azure_translator"),
            (ENV_PROVIDER_API_KEY_ENV, "fake-inline-key-value"),
            (ENV_ALLOW_REMOTE_PROVIDER, "true"),
        ],
    );

    assert_normalized_failure(
        &output,
        "PROVIDER_NOT_CONFIGURED",
        &["SOURCE_MARKER_PRIVATE", "fake-inline-key-value"],
    );
}

#[test]
fn rejected_local_response_has_normalized_process_output_and_preserves_fixture_hash() {
    let _port = OPERATIONAL_PORT.lock().expect("operational port lock");
    let raw_body = r#"{"error":"PRIVATE_RAW_RESPONSE PRIVATE_TRANSLATION PRIVATE_KEY"}"#;
    let url = common::operational_local_response_server_with_status(401, raw_body);
    let workspace = unique_temp_dir("rejected");
    fs::create_dir_all(&workspace).expect("workspace");
    let document = workspace.join("source.txt");
    let original = b"SOURCE_MARKER_PRIVATE\n";
    fs::write(&document, original).expect("fixture");
    let before_hash = fixture_hash(&document);
    let request = format!(
        r#"{{"file_path":"{}","workspace_root":"{}","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"}}"#,
        document.display(),
        workspace.display()
    );

    let output = run_cli(
        request,
        &[(ENV_PROVIDER, "libretranslate"), (ENV_PROVIDER_URL, &url)],
    );

    assert_normalized_failure(
        &output,
        "PROVIDER_FAILED",
        &[
            "SOURCE_MARKER_PRIVATE",
            "PRIVATE_RAW_RESPONSE",
            "PRIVATE_TRANSLATION",
            "PRIVATE_KEY",
            &document.display().to_string(),
            &workspace.display().to_string(),
        ],
    );
    assert_eq!(fixture_hash(&document), before_hash);
    assert_eq!(
        fs::read(&document).expect("fixture after failure"),
        original
    );
}

#[test]
fn malformed_local_response_is_reported_without_body_or_content() {
    let _port = OPERATIONAL_PORT.lock().expect("operational port lock");
    let url = common::operational_local_response_server(
        r#"{"PRIVATE_RAW_RESPONSE":"SOURCE_MARKER_PRIVATE"}"#,
    );
    let output = run_cli(
        request_json("SOURCE_MARKER_PRIVATE", "text"),
        &[(ENV_PROVIDER, "libretranslate"), (ENV_PROVIDER_URL, &url)],
    );

    assert_normalized_failure(
        &output,
        "PROVIDER_FAILED",
        &["SOURCE_MARKER_PRIVATE", "PRIVATE_RAW_RESPONSE", &url],
    );
}

#[test]
fn local_timeout_uses_the_stable_code_within_the_bounded_budget() {
    let _port = OPERATIONAL_PORT.lock().expect("operational port lock");
    let (url, server) = operational_timeout_server();
    let started = Instant::now();
    let output = run_cli(
        request_json("SOURCE_MARKER_PRIVATE", "text"),
        &[(ENV_PROVIDER, "libretranslate"), (ENV_PROVIDER_URL, &url)],
    );
    let elapsed = started.elapsed();
    server.join().expect("timeout server");

    assert_normalized_failure(
        &output,
        "PROVIDER_TIMEOUT",
        &["SOURCE_MARKER_PRIVATE", &url],
    );
    assert!(
        elapsed <= Duration::from_millis(PROVIDER_TIMEOUT_MS + 2_000),
        "CLI exceeded the provider timeout budget"
    );
}

fn request_json(source_text: &str, input_kind: &str) -> String {
    format!(
        r#"{{"source_text":"{source_text}","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"{input_kind}"}}"#
    )
}

fn run_cli(input: String, environment: &[(&str, &str)]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_translator-cli"));
    command
        .env_remove(ENV_PROVIDER)
        .env_remove(ENV_PROVIDER_URL)
        .env_remove(ENV_PROVIDER_API_KEY_ENV)
        .env_remove(ENV_ALLOW_REMOTE_PROVIDER)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (key, value) in environment {
        command.env(key, value);
    }
    let mut child = command.spawn().expect("spawn CLI");
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(input.as_bytes())
        .expect("write request");
    child.wait_with_output().expect("wait CLI")
}

fn run_azure_cli(input: String, key_value: Option<&str>) -> std::process::Output {
    let mut environment = vec![
        (ENV_PROVIDER, "azure_translator"),
        (ENV_PROVIDER_API_KEY_ENV, "AZURE_TRANSLATOR_TEST_KEY"),
        (ENV_ALLOW_REMOTE_PROVIDER, "true"),
    ];
    if key_value.is_none() {
        environment.pop();
        environment.pop();
        environment.push((ENV_ALLOW_REMOTE_PROVIDER, "true"));
    }

    let mut command = Command::new(env!("CARGO_BIN_EXE_translator-cli"));
    command
        .env_remove(ENV_PROVIDER_URL)
        .env_remove("AZURE_TRANSLATOR_TEST_KEY")
        .envs(environment)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(value) = key_value {
        command.env("AZURE_TRANSLATOR_TEST_KEY", value);
    }
    let mut child = command.spawn().expect("spawn Azure CLI");
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(input.as_bytes())
        .expect("write request");
    child.wait_with_output().expect("wait Azure CLI")
}

fn unique_temp_dir(case: &str) -> std::path::PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::temp_dir().join(format!("operational-provider-{case}-{nanos}"))
}

fn fixture_hash(path: &std::path::Path) -> u64 {
    let mut hasher = DefaultHasher::new();
    fs::read(path).expect("hash fixture").hash(&mut hasher);
    hasher.finish()
}

fn operational_timeout_server() -> (String, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:5000").expect("bind timeout server");
    let server = thread::spawn(move || {
        let (_stream, _) = listener.accept().expect("accept timeout request");
        thread::sleep(Duration::from_millis(PROVIDER_TIMEOUT_MS + 1_000));
    });
    ("http://127.0.0.1:5000".to_string(), server)
}

fn assert_normalized_failure(
    output: &std::process::Output,
    expected_code: &str,
    prohibited: &[&str],
) {
    let stdout = String::from_utf8(output.stdout.clone()).expect("stdout");
    let stderr = String::from_utf8(output.stderr.clone()).expect("stderr");

    assert_eq!(output.status.code(), Some(1));
    assert!(
        stdout.contains(&format!(r#""code":"{expected_code}""#)),
        "unexpected stdout: {stdout}"
    );
    assert_eq!(stderr, format!("error_code={expected_code}\n"));
    assert!(stdout.len() <= 192);
    assert!(stderr.len() <= 64);
    for marker in prohibited {
        assert!(
            !stdout.contains(marker),
            "stdout leaked a prohibited marker"
        );
        assert!(
            !stderr.contains(marker),
            "stderr leaked a prohibited marker"
        );
    }
}
