use translator_core::{translate_text, ErrorCode, MockProvider, MAX_INPUT_BYTES};

#[test]
fn rejects_direct_text_above_input_limit_before_provider_processing() {
    let input = "a".repeat(MAX_INPUT_BYTES + 1);

    let err =
        translate_text(&input, &MockProvider::new()).expect_err("oversized input should fail");

    assert_eq!(err.code, ErrorCode::FileTooLarge);
}
