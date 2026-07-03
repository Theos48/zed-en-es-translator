mod common;

#[test]
fn translate_text_returns_error_for_empty_text() {
    let value = common::translate_text_error_value(common::translate_text_params(""));

    common::assert_tool_error_code(&value, "INVALID_INPUT");
}

#[test]
fn translate_text_returns_error_for_whitespace_text() {
    let value = common::translate_text_error_value(common::translate_text_params("   \n\t"));

    common::assert_tool_error_code(&value, "INVALID_INPUT");
}

#[test]
fn translate_text_returns_error_for_unsupported_language() {
    let mut params = common::translate_text_params("Read the docs.");
    params.source_language = Some("fr".to_string());

    let value = common::translate_text_error_value(params);

    common::assert_tool_error_code(&value, "UNSUPPORTED_LANGUAGE_PAIR");
}

#[test]
fn translate_text_returns_error_when_preserve_formatting_is_false() {
    let mut params = common::translate_text_params("Read the docs.");
    params.preserve_formatting = Some(false);

    let value = common::translate_text_error_value(params);

    common::assert_tool_error_code(&value, "INVALID_INPUT");
}

#[test]
fn translate_text_returns_error_for_oversized_text() {
    let oversized = "x".repeat(translator_core::MAX_INPUT_BYTES + 1);
    let value = common::translate_text_error_value(common::translate_text_params(&oversized));

    common::assert_tool_error_code(&value, "FILE_TOO_LARGE");
}
