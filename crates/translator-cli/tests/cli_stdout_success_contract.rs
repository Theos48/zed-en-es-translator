use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn success_stdout_contains_no_logs_or_metadata() {
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
        .write_all(br#"{"source_text":"Open the file.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"}"#)
        .expect("write stdin");

    let output = child.wait_with_output().expect("wait");
    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");

    assert!(output.status.success());
    assert_eq!(stdout, r#"{"translated_text":"Abre el archivo."}"#);
    assert!(!stdout.contains("provider"));
    assert!(!stdout.contains("path"));
}
