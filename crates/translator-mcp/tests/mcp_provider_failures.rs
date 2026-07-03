use serde_json::Value;
use translator_core::{
    ErrorCode, Language, Provider, ProviderRequest, ProviderResponse, Tone, TranslateFailure,
    MAX_OUTPUT_BYTES,
};
use translator_mcp::protocol::TranslateTextParams;
use translator_mcp::tools::TranslatorMcpServer;

#[test]
fn maps_provider_failure_to_tool_error() {
    let value = translate_text_error_value(FailingProvider(ErrorCode::ProviderFailed));

    assert_error_code(&value, "PROVIDER_FAILED");
}

#[test]
fn maps_provider_timeout_to_tool_error() {
    let value = translate_text_error_value(FailingProvider(ErrorCode::ProviderTimeout));

    assert_error_code(&value, "PROVIDER_TIMEOUT");
}

#[test]
fn maps_malformed_provider_output_to_tool_error() {
    let value = translate_text_error_value(MalformedOutputProvider);

    assert_error_code(&value, "PROVIDER_FAILED");
}

#[test]
fn maps_output_limit_failure_to_tool_error() {
    let value = translate_text_error_value(OversizedOutputProvider);

    assert_error_code(&value, "PROVIDER_FAILED");
}

#[derive(Debug, Clone, Copy)]
struct FailingProvider(ErrorCode);

impl Provider for FailingProvider {
    fn translate(&self, _request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        Err(TranslateFailure::new(
            self.0,
            "provider detail must be redacted",
        ))
    }
}

#[derive(Debug, Clone, Copy)]
struct MalformedOutputProvider;

impl Provider for MalformedOutputProvider {
    fn translate(&self, _request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        Ok(ProviderResponse {
            translated_segments: Vec::new(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct OversizedOutputProvider;

impl Provider for OversizedOutputProvider {
    fn translate(&self, request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        assert_eq!(request.source_language, Language::English);
        assert_eq!(request.target_language, Language::Spanish);
        assert_eq!(request.tone, Tone::TechnicalNeutral);
        Ok(ProviderResponse {
            translated_segments: vec!["x".repeat(MAX_OUTPUT_BYTES + 1)],
        })
    }
}

fn translate_text_error_value(provider: impl Provider) -> Value {
    let result = TranslatorMcpServer::with_provider(provider).translate_text(TranslateTextParams {
        source_text: "Read the docs.".to_string(),
        source_language: Some("en".to_string()),
        target_language: Some("es".to_string()),
        tone: Some("technical_neutral".to_string()),
        preserve_formatting: Some(true),
    });
    serde_json::to_value(result).expect("serialize tool result")
}

fn assert_error_code(value: &Value, code: &str) {
    assert_eq!(value["isError"], true);
    assert_eq!(value["structuredContent"]["code"], code);
    assert!(!value.to_string().contains("provider detail"));
}
