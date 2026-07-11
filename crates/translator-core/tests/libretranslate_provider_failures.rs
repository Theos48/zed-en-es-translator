use translator_core::{translate_text, ErrorCode, LibreTranslateProvider, ProviderTarget};

mod common;

use common::StubHttpServer;

#[test]
fn maps_status_rejection_to_provider_failed() {
    let provider = provider_for_response(429, r#"{"error":"quota exceeded"}"#);

    let err = translate_text("Read the docs.", &provider).expect_err("status failure");

    assert_eq!(err.code, ErrorCode::ProviderFailed);
}

#[test]
fn maps_unsupported_language_pair_rejection_to_provider_failed() {
    let provider = provider_for_response(400, r#"{"error":"unsupported language pair"}"#);

    let err = translate_text("Read the docs.", &provider).expect_err("language failure");

    assert_eq!(err.code, ErrorCode::ProviderFailed);
}

#[test]
fn maps_malformed_response_to_provider_failed() {
    let provider = provider_for_response(200, r#"{"unexpected":"shape"}"#);

    let err = translate_text("Read the docs.", &provider).expect_err("malformed failure");

    assert_eq!(err.code, ErrorCode::ProviderFailed);
}

#[test]
fn maps_empty_response_text_to_provider_failed() {
    let provider = provider_for_response(200, r#"{"translatedText":[""]}"#);

    let err = translate_text("Read the docs.", &provider).expect_err("empty failure");

    assert_eq!(err.code, ErrorCode::ProviderFailed);
}

#[test]
fn maps_oversized_response_body_to_provider_failed() {
    let response_body = format!(
        r#"{{"translatedText":["{}"]}}"#,
        "a".repeat(translator_core::MAX_OUTPUT_BYTES + 4096)
    );
    let provider = provider_for_response(200, response_body);

    let err = translate_text("Read the docs.", &provider).expect_err("oversized failure");

    assert_eq!(err.code, ErrorCode::ProviderFailed);
}

fn provider_for_response(status: u16, response_body: impl Into<String>) -> LibreTranslateProvider {
    let server = StubHttpServer::with_status(status, response_body);
    LibreTranslateProvider::new(
        ProviderTarget::parse(&server.url(), false).expect("target"),
        None,
    )
}
