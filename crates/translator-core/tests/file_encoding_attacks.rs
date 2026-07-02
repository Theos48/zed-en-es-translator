use translator_core::{translate_file, ErrorCode, MockProvider};

mod common;
use common::{temp_case, write_file};

#[test]
fn rejects_nul_bytes_in_allowed_file() {
    let workspace = temp_case("nul");
    write_file(&workspace.join("binary.md"), b"Read\0the docs.");

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
    write_file(&workspace.join("mixed.txt"), b"Read the docs.\x01\x02");

    let err = translate_file(
        "mixed.txt",
        workspace.to_str().unwrap(),
        &MockProvider::new(),
    )
    .expect_err("mixed text/binary should fail");

    assert_eq!(err.code, ErrorCode::NonUtf8Input);
}
