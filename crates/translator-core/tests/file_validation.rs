use std::fs;
use std::path::PathBuf;

use translator_core::{translate_file, ErrorCode, MockProvider};

fn temp_case() -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "zed_translator_file_validation_{}_{}",
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
fn rejects_unsupported_file_type() {
    let workspace = temp_case();
    fs::write(workspace.join("data.json"), "{}").expect("write file");

    let err = translate_file(
        "data.json",
        workspace.to_str().unwrap(),
        &MockProvider::new(),
    )
    .expect_err("unsupported extension should fail");

    assert_eq!(err.code, ErrorCode::UnsupportedFileType);
}

#[test]
fn rejects_non_utf8_allowed_file() {
    let workspace = temp_case();
    fs::write(workspace.join("bad.md"), [0xff, 0xfe, 0xfd]).expect("write file");

    let err = translate_file("bad.md", workspace.to_str().unwrap(), &MockProvider::new())
        .expect_err("non-utf8 file should fail");

    assert_eq!(err.code, ErrorCode::NonUtf8Input);
}
