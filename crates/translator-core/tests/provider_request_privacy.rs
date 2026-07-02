use translator_core::{Language, ProviderRequest, Tone};

#[test]
fn provider_request_exposes_only_segments_language_pair_and_tone() {
    let request = ProviderRequest::new(
        vec!["Read the docs.".to_string()],
        Language::English,
        Language::Spanish,
        Tone::TechnicalNeutral,
    )
    .expect("provider request");

    let debug = format!("{request:?}");

    assert_eq!(request.segments, vec!["Read the docs."]);
    assert_eq!(request.source_language, Language::English);
    assert_eq!(request.target_language, Language::Spanish);
    assert_eq!(request.tone, Tone::TechnicalNeutral);
    assert!(!debug.contains("input_kind"));
    assert!(!debug.contains("workspace"));
    assert!(!debug.contains("file_path"));
}
