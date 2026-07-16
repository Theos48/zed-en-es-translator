use translator_core::{redact_failure, ErrorCode, TranslateFailure};

#[test]
fn embedded_failure_should_redact_paths_and_urls() {
    let failure = TranslateFailure::new(
        ErrorCode::ProviderFailed,
        "failed /home/private/model at https://sensitive.invalid/model",
    );

    let redacted = redact_failure(failure);

    assert_eq!(redacted.message, "The provider failed.");
}
