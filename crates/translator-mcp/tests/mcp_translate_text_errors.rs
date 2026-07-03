use serde_json::Value;
use translator_mcp::protocol::TranslateTextParams;
use translator_mcp::tools::TranslatorMcpServer;

#[test]
fn translate_text_returns_error_for_empty_text() {
    let value = translate_text_error_value(params(""));

    assert_error_code(&value, "INVALID_INPUT");
}

#[test]
fn translate_text_returns_error_for_whitespace_text() {
    let value = translate_text_error_value(params("   \n\t"));

    assert_error_code(&value, "INVALID_INPUT");
}

#[test]
fn translate_text_returns_error_for_unsupported_language() {
    let mut params = params("Read the docs.");
    params.source_language = Some("fr".to_string());

    let value = translate_text_error_value(params);

    assert_error_code(&value, "UNSUPPORTED_LANGUAGE_PAIR");
}

#[test]
fn translate_text_returns_error_when_preserve_formatting_is_false() {
    let mut params = params("Read the docs.");
    params.preserve_formatting = Some(false);

    let value = translate_text_error_value(params);

    assert_error_code(&value, "INVALID_INPUT");
}

#[test]
fn translate_text_returns_error_for_oversized_text() {
    let oversized = "x".repeat(translator_core::MAX_INPUT_BYTES + 1);
    let value = translate_text_error_value(params(&oversized));

    assert_error_code(&value, "FILE_TOO_LARGE");
}

fn params(source_text: &str) -> TranslateTextParams {
    TranslateTextParams {
        source_text: source_text.to_string(),
        source_language: Some("en".to_string()),
        target_language: Some("es".to_string()),
        tone: Some("technical_neutral".to_string()),
        preserve_formatting: Some(true),
    }
}

fn translate_text_error_value(params: TranslateTextParams) -> Value {
    let result = TranslatorMcpServer::new().translate_text(params);
    serde_json::to_value(result).expect("serialize tool result")
}

fn assert_error_code(value: &Value, code: &str) {
    assert_eq!(value["isError"], true);
    assert_eq!(value["structuredContent"]["code"], code);
}
