use std::io::Write;
use std::process::{Command, Stdio};

use translator_core::{ENV_PROVIDER, ENV_PROVIDER_URL};

mod common;

#[test]
fn cli_redacts_provider_failure_stdout_and_stderr() {
    let url = common::response_server_with_status(
        429,
        r#"{"error":"source_text=Read the docs. Authorization: Bearer fake_token"}"#,
    );
    let mut child = Command::new(env!("CARGO_BIN_EXE_translator-cli"))
        .env(ENV_PROVIDER, "libretranslate")
        .env(ENV_PROVIDER_URL, url)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn translator-cli");
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(br#"{"source_text":"Read the docs.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"}"#)
        .expect("write stdin");

    let output = child.wait_with_output().expect("wait");
    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
    let stderr = String::from_utf8(output.stderr).expect("stderr utf-8");

    assert!(!output.status.success());
    assert!(stdout.contains(r#""code":"PROVIDER_FAILED""#));
    assert!(stderr.contains("PROVIDER_FAILED"));
    assert!(!stdout.contains("fake_token"));
    assert!(!stderr.contains("fake_token"));
    assert!(!stdout.contains("Read the docs."));
    assert!(!stderr.contains("Read the docs."));
}
