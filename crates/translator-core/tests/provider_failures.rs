use translator_core::{
    translate_text, ErrorCode, Provider, ProviderRequest, ProviderResponse, TranslateFailure,
};

struct FailingProvider {
    code: ErrorCode,
}

impl Provider for FailingProvider {
    fn translate(&self, _request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        Err(TranslateFailure::new(self.code, "provider failed"))
    }
}

#[test]
fn maps_provider_failure() {
    let err = translate_text(
        "Read the docs.",
        &FailingProvider {
            code: ErrorCode::ProviderFailed,
        },
    )
    .expect_err("provider failure should fail");

    assert_eq!(err.code, ErrorCode::ProviderFailed);
}

#[test]
fn maps_provider_timeout() {
    let err = translate_text(
        "Read the docs.",
        &FailingProvider {
            code: ErrorCode::ProviderTimeout,
        },
    )
    .expect_err("provider timeout should fail");

    assert_eq!(err.code, ErrorCode::ProviderTimeout);
}
