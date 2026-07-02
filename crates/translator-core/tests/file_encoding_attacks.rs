use std::fs;
use std::path::PathBuf;

use translator_core::{translate_file, ErrorCode, MockProvider};

fn temp_case(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "zed_translator_encoding_{}_{}_{}",
        name,
        std::process::id(),
        unique_suffix()
    ));
    fs::create_dir_all(&root).expect("temp root");
    root
}

fn unique_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time")
        .as_nanos()
}

#[test]
fn rejects_nul_bytes_in_allowed_file() {
    let workspace = temp_case("nul");
    fs::write(workspace.join("binary.md"), b"Read\0the docs.").expect("write file");

    let err = translate_file(
        "binary.md",
        workspace.to_str().unwrap(),
        &MockProvider::new(),
    )
    .expect_err("NUL bytes should fail");

    assert_eq!(err.code, ErrorCode::NonUtf8Input);
}

#[test]
fn rejects_mixed_text_binary_payload() {
    let workspace = temp_case("mixed");
    fs::write(workspace.join("mixed.txt"), b"Read the docs.\x01\x02").expect("write file");

    let err = translate_file(
        "mixed.txt",
        workspace.to_str().unwrap(),
        &MockProvider::new(),
    )
    .expect_err("mixed text/binary should fail");

    assert_eq!(err.code, ErrorCode::NonUtf8Input);
}
