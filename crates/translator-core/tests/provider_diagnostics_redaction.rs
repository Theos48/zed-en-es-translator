use translator_core::{redact_failure, ErrorCode, TranslateFailure};

#[test]
fn redacts_provider_failure_detail() {
    let failure = TranslateFailure::new(
        ErrorCode::ProviderFailed,
        "provider body source_text=Read the docs. Authorization: Bearer fake_token",
    );

    let redacted = redact_failure(failure);

    assert_eq!(redacted.message, "The provider failed.");
}

#[test]
fn redacts_provider_timeout_detail() {
    let failure = TranslateFailure::new(
        ErrorCode::ProviderTimeout,
        "timeout contacting https://example.invalid/translate",
    );

    let redacted = redact_failure(failure);

    assert_eq!(redacted.message, "The provider timed out.");
}
