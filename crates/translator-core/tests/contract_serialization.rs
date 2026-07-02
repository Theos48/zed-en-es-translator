use translator_core::{
    ErrorCode, InputKind, Language, Tone, TranslateFailure, TranslateRequest, TranslateResult,
    TranslateSuccess,
};

#[test]
fn serializes_direct_text_request_to_stable_wire_json() {
    let request = TranslateRequest::direct_text("Read the docs.");

    assert_eq!(
        request.to_json(),
        r#"{"source_text":"Read the docs.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"}"#
    );
}

#[test]
fn serializes_file_request_without_source_text() {
    let request =
        TranslateRequest::file("docs/readme.md", "/workspace/project", InputKind::Markdown);

    assert_eq!(request.source_text, None);
    assert_eq!(
        request.to_json(),
        r#"{"source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"markdown","file_path":"docs/readme.md","workspace_root":"/workspace/project"}"#
    );
}

#[test]
fn parses_direct_text_request_from_wire_json() {
    let request = TranslateRequest::from_json(
        r#"{"source_text":"Read the docs.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"}"#,
    )
    .expect("valid direct text request");

    assert_eq!(request.source_text.as_deref(), Some("Read the docs."));
    assert_eq!(request.source_language, Language::English);
    assert_eq!(request.target_language, Language::Spanish);
    assert_eq!(request.tone, Tone::TechnicalNeutral);
    assert_eq!(request.input_kind, InputKind::Text);
    assert!(request.preserve_formatting);
}

#[test]
fn serializes_success_and_failure_results_to_stable_wire_json() {
    let success = TranslateResult::Success(TranslateSuccess {
        translated_text: "Lee la documentacion.".to_string(),
    });

    let failure = TranslateResult::Failure(TranslateFailure::new(
        ErrorCode::FileTooLarge,
        "The input exceeds the configured size limit.",
    ));

    assert_eq!(
        success.to_json(),
        r#"{"translated_text":"Lee la documentacion."}"#
    );
    assert_eq!(
        failure.to_json(),
        r#"{"code":"FILE_TOO_LARGE","message":"The input exceeds the configured size limit."}"#
    );
}
