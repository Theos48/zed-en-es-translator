use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn failure_uses_nonzero_exit_redacted_stderr_and_failure_stdout() {
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
        .write_all(br#"{"source_text":"","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"}"#)
        .expect("write stdin");

    let output = child.wait_with_output().expect("wait");
    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
    let stderr = String::from_utf8(output.stderr).expect("stderr utf-8");

    assert!(!output.status.success());
    assert!(stdout.contains(r#""code":"INVALID_INPUT""#));
    assert!(stderr.contains("INVALID_INPUT"));
    assert!(!stderr.contains("source_text"));
}
