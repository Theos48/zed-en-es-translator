use std::io::Write;
use std::process::{Command, Stdio};

fn run_cli(input: &str, args: &[&str]) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_translator-cli"))
        .args(args)
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

#[test]
fn cli_accepts_json_on_stdin_and_returns_success_json_on_stdout() {
    let output = run_cli(
        r#"{"source_text":"Read the docs.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"}"#,
        &[],
    );

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("stdout utf-8"),
        r#"{"translated_text":"Lee la documentacion."}"#
    );
    assert_eq!(String::from_utf8(output.stderr).expect("stderr utf-8"), "");
}
