use std::io::Write;
use std::process::{Command, Stdio};

fn run_cli(input: &str) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_translator-cli"))
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

fn run_cli_bytes(input: &[u8]) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_translator-cli"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn translator-cli");

    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(input)
        .expect("write stdin");

    child.wait_with_output().expect("wait for CLI")
}

#[test]
fn cli_rejects_malformed_json() {
    let output = run_cli("{not-json");

    assert!(!output.status.success());
    assert!(String::from_utf8(output.stdout)
        .expect("stdout utf-8")
        .contains(r#""code":"INVALID_INPUT""#));
}

#[test]
fn cli_rejects_unknown_fields_and_wrong_types() {
    for input in [
        r#"{"source_text":"Read.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text","provider":"remote"}"#,
        r#"{"source_text":123,"source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"}"#,
        r#"{"source_text":"Read.","source_language":"fr","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"}"#,
        r#"{"source_text":"Read.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":false,"input_kind":"text"}"#,
        r#"{"source_text":"Read.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text","file_path":"README.md","workspace_root":"/workspace"}"#,
    ] {
        let output = run_cli(input);
        assert!(!output.status.success(), "input should fail: {input}");
    }
}

#[test]
fn cli_rejects_stdin_above_configured_limit_before_full_json_parse() {
    let filler = "a".repeat(translator_core::MAX_INPUT_BYTES + 1);
    let oversized = format!(
        r#"{{"source_text":"{filler}","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"}}"#
    );
    let output = run_cli_bytes(oversized.as_bytes());

    assert!(!output.status.success());
    assert!(String::from_utf8(output.stdout)
        .expect("stdout utf-8")
        .contains(r#""code":"INVALID_INPUT""#));
}
