use std::sync::{Arc, Mutex};

use translator_core::{
    AzureTranslatorProvider, AzureTransport, AzureTransportError, Language, Provider,
    ProviderRequest, Tone, AZURE_TRANSLATOR_ENDPOINT,
};

#[derive(Clone)]
struct RecordingTransport {
    bodies: Arc<Mutex<Vec<Vec<u8>>>>,
    response: Vec<u8>,
}

impl RecordingTransport {
    fn new(response: &str) -> Self {
        Self {
            bodies: Arc::new(Mutex::new(Vec::new())),
            response: response.as_bytes().to_vec(),
        }
    }

    fn first_body(&self) -> Vec<u8> {
        self.bodies.lock().expect("bodies")[0].clone()
    }
}

impl AzureTransport for RecordingTransport {
    fn send(&self, body: &[u8]) -> Result<Vec<u8>, AzureTransportError> {
        self.bodies.lock().expect("bodies").push(body.to_vec());
        Ok(self.response.clone())
    }
}

#[test]
fn exact_endpoint_is_fixed_inside_the_adapter() {
    assert_eq!(
        AZURE_TRANSLATOR_ENDPOINT,
        "https://api.cognitive.microsofttranslator.com/translate?api-version=3.0&from=en&to=es"
    );
}

#[test]
fn sends_ordered_segments_as_text_only_elements() {
    let transport = RecordingTransport::new(
        r#"[{"translations":[{"text":"Lee.","to":"es"}]},{"translations":[{"text":"Abre.","to":"es"}]}]"#,
    );
    let provider = AzureTranslatorProvider::with_transport(transport.clone());
    let request = ProviderRequest::new(
        vec!["Read.".to_string(), "Open.".to_string()],
        Language::English,
        Language::Spanish,
        Tone::TechnicalNeutral,
    )
    .expect("request");

    let response = provider.translate(&request).expect("translation");

    assert_eq!(response.translated_segments, ["Lee.", "Abre."]);
    let body: serde_json::Value =
        serde_json::from_slice(&transport.first_body()).expect("request JSON");
    assert_eq!(body, serde_json::json!([{"Text":"Read."},{"Text":"Open."}]));
}

#[test]
fn rejects_unsupported_internal_invariants_before_transport_contact() {
    let transport = RecordingTransport::new("[]");
    let provider = AzureTranslatorProvider::with_transport(transport.clone());
    let mut request = ProviderRequest::new(
        vec!["Read.".to_string()],
        Language::English,
        Language::Spanish,
        Tone::TechnicalNeutral,
    )
    .expect("request");
    request.preserve_formatting = false;

    let error = provider
        .translate(&request)
        .expect_err("formatting invariant must fail");

    assert_eq!(error.code, translator_core::ErrorCode::InvalidInput);
    assert!(transport.bodies.lock().expect("bodies").is_empty());
}

#[test]
fn production_transport_contract_disables_proxy_redirect_and_extra_headers() {
    let source = include_str!("../src/azure_translator.rs");

    for required in [
        ".proxy(None)",
        ".max_redirects(0)",
        "Ocp-Apim-Subscription-Key",
        "Content-Type",
    ] {
        assert!(
            source.contains(required),
            "missing transport control: {required}"
        );
    }
    for forbidden in [
        "Ocp-Apim-Subscription-Region",
        "preserve_formatting\"",
        "tone\"",
    ] {
        assert!(
            !source.contains(forbidden),
            "invented external metadata: {forbidden}"
        );
    }
}
