use translator_core::{translate_text, ErrorCode, MockProvider};

#[test]
fn rejects_empty_direct_text() {
    let err = translate_text("", &MockProvider::new()).expect_err("empty input should fail");

    assert_eq!(err.code, ErrorCode::InvalidInput);
}

#[test]
fn rejects_whitespace_only_direct_text() {
    let err =
        translate_text(" \n\t ", &MockProvider::new()).expect_err("whitespace input should fail");

    assert_eq!(err.code, ErrorCode::InvalidInput);
}
