use std::process::Command;

#[test]
fn cli_rejects_argv_text_without_echoing_it() {
    let output = Command::new(env!("CARGO_BIN_EXE_translator-cli"))
        .arg("Authorization: Bearer fake_test_token")
        .output()
        .expect("run translator-cli");

    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
    let stderr = String::from_utf8(output.stderr).expect("stderr utf-8");

    assert!(!output.status.success());
    assert!(stdout.contains(r#""code":"INVALID_INPUT""#));
    assert!(!stdout.contains("Bearer"));
    assert!(!stderr.contains("Bearer"));
}
