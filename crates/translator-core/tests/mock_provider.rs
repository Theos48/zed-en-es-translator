use translator_core::{Language, MockProvider, Provider, ProviderRequest, Tone};

#[test]
fn mock_provider_is_deterministic() {
    let provider = MockProvider::new();
    let request = ProviderRequest::new(
        vec!["Read the docs.".to_string(), "Open the file.".to_string()],
        Language::English,
        Language::Spanish,
        Tone::TechnicalNeutral,
    )
    .expect("provider request");

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
