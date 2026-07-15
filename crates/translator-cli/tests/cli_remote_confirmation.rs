use std::io::Write;
use std::process::{Command, Stdio};

use translator_core::{
    ENV_ALLOW_REMOTE_PROVIDER, ENV_PROVIDER, ENV_PROVIDER_API_KEY_ENV, ENV_PROVIDER_URL,
};

#[test]
fn cli_denies_unconfirmed_non_local_provider() {
    let output = run_cli(remote_request("Read the docs.", false));

    assert!(!output.status.success());
    assert!(String::from_utf8(output.stdout)
        .expect("stdout utf-8")
        .contains(r#""code":"REMOTE_CONFIRMATION_REQUIRED""#));
}

#[test]
fn cli_blocks_confirmed_non_local_secret_before_contact() {
    let output = run_cli(remote_request(
        "API_KEY=fake_test_key_123456 should stay local.",
        true,
    ));
    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
    let stderr = String::from_utf8(output.stderr).expect("stderr utf-8");

    assert!(!output.status.success());
    assert!(stdout.contains(r#""code":"SECRET_DETECTED""#));
    assert!(!stdout.contains("fake_test_key"));
    assert!(!stderr.contains("fake_test_key"));
}

fn remote_request(source_text: &str, remote_confirmed: bool) -> String {
    let suffix = if remote_confirmed {
        r#","remote_confirmed":true"#
    } else {
        ""
    };
    format!(
        r#"{{"source_text":"{source_text}","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"{suffix}}}"#
    )
}

fn run_cli(input: String) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_translator-cli"))
        .env(ENV_PROVIDER, "azure_translator")
        .env_remove(ENV_PROVIDER_URL)
        .env(ENV_PROVIDER_API_KEY_ENV, "AZURE_TRANSLATOR_TEST_KEY")
        .env(ENV_ALLOW_REMOTE_PROVIDER, "true")
        .env("AZURE_TRANSLATOR_TEST_KEY", "fake-public-test-key")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn translator-cli");
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(input.as_bytes())
        .expect("write stdin");
    child.wait_with_output().expect("wait for CLI")
}
