use translator_core::{
    LibreTranslateProvider, Provider, ProviderConfiguration, ProviderRequest, ProviderTarget,
};

mod common;

use common::StubHttpServer;

#[test]
fn sends_only_permitted_payload_fields() {
    let server = StubHttpServer::new(r#"{"translatedText":["Lee la documentacion."]}"#);
    let provider = LibreTranslateProvider::new(
        ProviderTarget::parse(&server.url(), false).expect("target"),
        None,
    );
    let request = ProviderRequest::new(
        vec!["Read the docs.".to_string()],
        translator_core::Language::English,
        translator_core::Language::Spanish,
        translator_core::Tone::TechnicalNeutral,
    )
    .expect("request");

    let response = provider.translate(&request).expect("translation");

    assert_eq!(response.translated_segments, vec!["Lee la documentacion."]);
    let body = server.first_body();
    let payload: serde_json::Value = serde_json::from_str(&body).expect("json body");
    assert_eq!(payload["q"], serde_json::json!(["Read the docs."]));
    assert_eq!(payload["source"], "en");
    assert_eq!(payload["target"], "es");
    assert_eq!(payload["format"], "text");
    assert!(payload.get("workspace").is_none());
    assert!(payload.get("file_path").is_none());
    assert!(payload.get("headers").is_none());
}

#[test]
fn batches_multiple_segments_in_one_provider_request() {
    let server =
        StubHttpServer::new(r#"{"translatedText":["Lee la documentacion.","Abre el archivo."]}"#);
    let provider = LibreTranslateProvider::new(
        ProviderTarget::parse(&server.url(), false).expect("target"),
        None,
    );
    let request = ProviderRequest::new(
        vec!["Read the docs.".to_string(), "Open the file.".to_string()],
        translator_core::Language::English,
        translator_core::Language::Spanish,
        translator_core::Tone::TechnicalNeutral,
    )
    .expect("request");

    let response = provider.translate(&request).expect("translation");

    assert_eq!(
        response.translated_segments,
        vec!["Lee la documentacion.", "Abre el archivo."]
    );
    let bodies = server.bodies(1);
    assert_eq!(bodies.len(), 1);
    let payload: serde_json::Value = serde_json::from_str(&bodies[0]).expect("json body");
    assert_eq!(
        payload["q"],
        serde_json::json!(["Read the docs.", "Open the file."])
    );
}

#[test]
fn rejects_proxy_environment_as_configuration_source() {
    let config = ProviderConfiguration::from_values(
        Some("libretranslate"),
        Some("http://127.0.0.1:5000"),
        None,
        None,
    )
    .expect("config");

    assert!(!config.uses_inherited_proxy_environment());
}
