use translator_core::{
    translate_text, ErrorCode, Provider, ProviderRequest, ProviderResponse, TranslateFailure,
};

struct FailingProvider;

impl Provider for FailingProvider {
    fn translate(&self, _request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        Err(TranslateFailure::new(
            ErrorCode::InternalError,
            "Provider should not receive ambiguous code-like content.",
        ))
    }
}

#[test]
fn preserves_ambiguous_code_like_direct_text_without_provider() {
    let input = r#"std::process::Command::new("rm")"#;

    let success = translate_text(input, &FailingProvider).expect("ambiguous content is preserved");

    assert_eq!(success.translated_text, input);
}
