use translator_core::{
    translate_text, ErrorCode, Provider, ProviderRequest, ProviderResponse, TranslateFailure,
};

struct LeakyFailingProvider;

impl Provider for LeakyFailingProvider {
    fn translate(&self, _request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        Err(TranslateFailure::new(
            ErrorCode::ProviderFailed,
            "failed while processing Read the docs. with Authorization: Bearer fake_test_token at /home/theos/private/secret.md",
        ))
    }
}

#[test]
fn redacts_provider_failure_diagnostics() {
    let err = translate_text("Read the docs.", &LeakyFailingProvider)
        .expect_err("provider failure should be redacted");

    assert_eq!(err.code, ErrorCode::ProviderFailed);
    assert!(!err.message.contains("Read the docs"));
    assert!(!err.message.contains("Bearer"));
    assert!(!err.message.contains("/home/theos"));
}
