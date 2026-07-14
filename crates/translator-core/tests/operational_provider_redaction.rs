use translator_core::{
    redact_failure, AzureTranslatorProvider, AzureTransport, AzureTransportError, ErrorCode,
    Language, LibreTranslateProvider, Provider, ProviderRequest, ProviderResponse, ProviderTarget,
    Tone,
};

mod common;

use common::StubHttpServer;

#[derive(Clone)]
struct FailingTransport;

impl AzureTransport for FailingTransport {
    fn send(&self, _body: &[u8]) -> Result<Vec<u8>, AzureTransportError> {
        Err(AzureTransportError::Failed)
    }
}

#[test]
fn provider_request_and_response_debug_hide_content() {
    let request = ProviderRequest::new(
        vec!["SOURCE_MARKER_PRIVATE".to_string()],
        Language::English,
        Language::Spanish,
        Tone::TechnicalNeutral,
    )
    .expect("request");
    let response = ProviderResponse {
        translated_segments: vec!["TRANSLATION_MARKER_PRIVATE".to_string()],
    };

    let debug = format!("{request:?} {response:?}");

    assert!(!debug.contains("SOURCE_MARKER_PRIVATE"));
    assert!(!debug.contains("TRANSLATION_MARKER_PRIVATE"));
}

#[test]
fn azure_failures_expose_only_generic_redacted_status() {
    let provider = AzureTranslatorProvider::with_transport(FailingTransport);
    let request = ProviderRequest::new(
        vec!["SOURCE_MARKER_PRIVATE".to_string()],
        Language::English,
        Language::Spanish,
        Tone::TechnicalNeutral,
    )
    .expect("request");

    let error = provider
        .translate(&request)
        .expect_err("controlled failure");
    let debug = format!("{error:?}");

    assert!(!debug.contains("SOURCE_MARKER_PRIVATE"));
    assert!(!debug.contains("api.cognitive"));
    assert!(!debug.contains("Ocp-Apim"));
}

#[test]
fn cross_provider_failures_use_bounded_stable_redacted_contracts() {
    let raw_body = "PRIVATE_RAW_RESPONSE_BODY Authorization: Bearer PRIVATE_TOKEN";
    let local_server = StubHttpServer::with_status(503, raw_body);
    let local = LibreTranslateProvider::new(
        ProviderTarget::parse(&local_server.url(), false).expect("local target"),
        None,
    );
    let local_failure = local.translate(&request()).expect_err("local rejection");

    for (transport_error, expected_code) in [
        (AzureTransportError::Timeout, ErrorCode::ProviderTimeout),
        (AzureTransportError::Http408, ErrorCode::ProviderTimeout),
        (AzureTransportError::Rejected, ErrorCode::ProviderFailed),
        (AzureTransportError::BodyTooLarge, ErrorCode::ProviderFailed),
    ] {
        let remote = AzureTranslatorProvider::with_transport(FixedFailureTransport {
            error: transport_error,
        });
        let remote_failure = remote.translate(&request()).expect_err("remote failure");
        assert_safe_failure(remote_failure, expected_code, raw_body);
    }

    assert_safe_failure(local_failure, ErrorCode::ProviderFailed, raw_body);
}

#[test]
fn provider_debug_omits_target_and_credential_reference() {
    let private_target = "https://private-provider.invalid/sensitive/path";
    let private_reference = "PRIVATE_PROVIDER_KEY_REFERENCE";
    let provider = LibreTranslateProvider::new(
        ProviderTarget::parse(private_target, true).expect("reviewed target"),
        Some(private_reference.to_string()),
    );

    let debug = format!("{provider:?}");

    assert!(!debug.contains(private_target));
    assert!(!debug.contains("private-provider"));
    assert!(!debug.contains(private_reference));
    assert!(!debug.contains("ureq"));
}

#[derive(Clone, Copy)]
struct FixedFailureTransport {
    error: AzureTransportError,
}

impl AzureTransport for FixedFailureTransport {
    fn send(&self, _body: &[u8]) -> Result<Vec<u8>, AzureTransportError> {
        Err(self.error)
    }
}

fn request() -> ProviderRequest {
    ProviderRequest::new(
        vec!["SOURCE_MARKER_PRIVATE".to_string()],
        Language::English,
        Language::Spanish,
        Tone::TechnicalNeutral,
    )
    .expect("request")
}

fn assert_safe_failure(
    failure: translator_core::TranslateFailure,
    expected_code: ErrorCode,
    raw_marker: &str,
) {
    let failure = redact_failure(failure);
    let debug = format!("{failure:?}");

    assert_eq!(failure.code, expected_code);
    assert!(failure.message.len() <= 96);
    for prohibited in [
        "SOURCE_MARKER_PRIVATE",
        "TRANSLATION_MARKER_PRIVATE",
        "Authorization",
        "PRIVATE_TOKEN",
        raw_marker,
        "http://",
        "https://",
    ] {
        assert!(!failure.message.contains(prohibited));
        assert!(!debug.contains(prohibited));
    }
}
