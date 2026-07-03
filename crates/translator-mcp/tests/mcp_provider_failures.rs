mod common;

use translator_core::{
    ErrorCode, Language, Provider, ProviderRequest, ProviderResponse, Tone, TranslateFailure,
    MAX_OUTPUT_BYTES,
};

#[test]
fn maps_provider_failure_to_tool_error() {
    let value = common::translate_text_error_value_with_provider(FailingProvider(
        ErrorCode::ProviderFailed,
    ));

    common::assert_tool_error_code_redacts(&value, "PROVIDER_FAILED", "provider detail");
}

#[test]
fn maps_provider_timeout_to_tool_error() {
    let value = common::translate_text_error_value_with_provider(FailingProvider(
        ErrorCode::ProviderTimeout,
    ));

    common::assert_tool_error_code_redacts(&value, "PROVIDER_TIMEOUT", "provider detail");
}

#[test]
fn maps_malformed_provider_output_to_tool_error() {
    let value = common::translate_text_error_value_with_provider(MalformedOutputProvider);

    common::assert_tool_error_code(&value, "PROVIDER_FAILED");
}

#[test]
fn maps_output_limit_failure_to_tool_error() {
    let value = common::translate_text_error_value_with_provider(OversizedOutputProvider);

    common::assert_tool_error_code(&value, "PROVIDER_FAILED");
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
