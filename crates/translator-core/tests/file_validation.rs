use translator_core::{translate_file, ErrorCode, MockProvider};

mod common;
use common::{temp_case, write_file};

#[test]
fn rejects_unsupported_file_type() {
    let workspace = temp_case("file_validation");
    write_file(&workspace.join("data.json"), "{}");

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
    let workspace = temp_case("non_utf8");
    write_file(&workspace.join("bad.md"), [0xff, 0xfe, 0xfd]);

    let err = translate_file("bad.md", workspace.to_str().unwrap(), &MockProvider::new())
        .expect_err("non-utf8 file should fail");

    assert_eq!(err.code, ErrorCode::NonUtf8Input);
}
