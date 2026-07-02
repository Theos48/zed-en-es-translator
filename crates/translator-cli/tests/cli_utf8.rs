use std::io::Write;
use std::process::{Command, Stdio};

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
fn cli_preserves_raw_utf8_json_strings() {
    let output = run_cli_bytes(
        r#"{"source_text":"Read café docs.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"}"#.as_bytes(),
    );

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
    assert!(stdout.contains("café"));
    assert!(!stdout.contains("cafÃ"));
}

#[test]
fn cli_maps_invalid_utf8_stdin_to_non_utf8_input() {
    let output = run_cli_bytes(&[b'{', b'"', 0xff, b'"', b':', b't', b'r', b'u', b'e', b'}']);

    assert!(!output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
    assert!(stdout.contains(r#""code":"NON_UTF8_INPUT""#));
}
