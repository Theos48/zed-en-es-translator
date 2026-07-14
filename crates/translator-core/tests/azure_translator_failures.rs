use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use translator_core::{
    AzureTranslatorProvider, AzureTransport, AzureTransportError, ErrorCode, Language, Provider,
    ProviderRequest, Tone, MAX_OUTPUT_BYTES,
};

#[derive(Clone)]
struct FixedTransport {
    result: Result<Vec<u8>, AzureTransportError>,
    calls: Arc<AtomicUsize>,
}

impl AzureTransport for FixedTransport {
    fn send(&self, _body: &[u8]) -> Result<Vec<u8>, AzureTransportError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.result.clone()
    }
}

#[test]
fn transport_failures_map_to_stable_codes_without_retry() {
    for (failure, expected) in [
        (AzureTransportError::Timeout, ErrorCode::ProviderTimeout),
        (AzureTransportError::Http408, ErrorCode::ProviderTimeout),
        (AzureTransportError::Dns, ErrorCode::ProviderFailed),
        (AzureTransportError::Tls, ErrorCode::ProviderFailed),
        (AzureTransportError::Redirect, ErrorCode::ProviderFailed),
        (AzureTransportError::Rejected, ErrorCode::ProviderFailed),
        (AzureTransportError::BodyTooLarge, ErrorCode::ProviderFailed),
        (AzureTransportError::Failed, ErrorCode::ProviderFailed),
    ] {
        let calls = Arc::new(AtomicUsize::new(0));
        let provider = AzureTranslatorProvider::with_transport(FixedTransport {
            result: Err(failure),
            calls: Arc::clone(&calls),
        });

        let error = provider
            .translate(&request())
            .expect_err("transport failure");

        assert_eq!(error.code, expected);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}

#[test]
fn invalid_response_families_fail_closed() {
    for body in [
        b"not-json".to_vec(),
        b"{}".to_vec(),
        b"[]".to_vec(),
        br#"[{"translations":[]}]"#.to_vec(),
        br#"[{"translations":[{"text":"","to":"es"}]}]"#.to_vec(),
        br#"[{"translations":[{"text":7,"to":"es"}]}]"#.to_vec(),
        br#"[{"translations":[{"text":"Texto","to":"fr"}]}]"#.to_vec(),
        br#"[{"translations":[{"text":"Uno","to":"es"}]},{"translations":[{"text":"Dos","to":"es"}]}]"#.to_vec(),
    ] {
        let provider = provider_with_body(body);

        let error = provider
            .translate(&request())
            .expect_err("invalid response must fail");

        assert_eq!(error.code, ErrorCode::ProviderFailed);
    }
}

#[test]
fn aggregate_output_above_limit_is_rejected() {
    let body = format!(
        r#"[{{"translations":[{{"text":"{}","to":"es"}}]}}]"#,
        "x".repeat(MAX_OUTPUT_BYTES + 1)
    )
    .into_bytes();
    let provider = provider_with_body(body);

    let error = provider
        .translate(&request())
        .expect_err("oversized output must fail");

    assert_eq!(error.code, ErrorCode::ProviderFailed);
}

fn provider_with_body(body: Vec<u8>) -> AzureTranslatorProvider<FixedTransport> {
    AzureTranslatorProvider::with_transport(FixedTransport {
        result: Ok(body),
        calls: Arc::new(AtomicUsize::new(0)),
    })
}

fn request() -> ProviderRequest {
    ProviderRequest::new(
        vec!["Read.".to_string()],
        Language::English,
        Language::Spanish,
        Tone::TechnicalNeutral,
    )
    .expect("request")
}
