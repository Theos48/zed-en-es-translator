use translator_core::{InputKind, Language, MockProvider, Provider, ProviderRequest, Tone};

#[test]
fn mock_provider_is_deterministic() {
    let provider = MockProvider::new();
    let request = ProviderRequest {
        segments: vec!["Read the docs.".to_string(), "Open the file.".to_string()],
        source_language: Language::English,
        target_language: Language::Spanish,
        tone: Tone::TechnicalNeutral,
        input_kind: InputKind::Text,
    };

    let first = provider
        .translate(&request)
        .expect("first provider response");
    let second = provider
        .translate(&request)
        .expect("second provider response");

    assert_eq!(first, second);
    assert_eq!(first.translated_segments.len(), request.segments.len());
    assert_eq!(first.translated_segments[0], "Lee la documentacion.");
    assert_eq!(first.translated_segments[1], "Abre el archivo.");
}
