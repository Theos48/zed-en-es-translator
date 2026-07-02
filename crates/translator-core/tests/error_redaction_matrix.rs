use translator_core::{redact_failure, ErrorCode, TranslateFailure};

#[test]
fn redacts_sensitive_content_for_all_error_classes() {
    for code in ErrorCode::ALL {
        let failure = TranslateFailure::new(
            code,
            "Read the docs. Authorization: Bearer fake_test_token API_KEY=fake /home/theos/private/secret.md",
        );
        let redacted = redact_failure(failure);

        assert_eq!(redacted.code, code);
        assert!(!redacted.message.contains("Read the docs"));
        assert!(!redacted.message.contains("Bearer"));
        assert!(!redacted.message.contains("API_KEY"));
        assert!(!redacted.message.contains("/home/theos"));
    }
}

#[test]
fn redacts_macos_home_paths() {
    let redacted = translator_core::redact_text("/Users/theos/private/secret.md");

    assert_eq!(redacted, "[REDACTED]");
}
