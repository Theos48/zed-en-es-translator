use translator_core::TranslateRequest;

#[test]
fn parses_raw_utf8_json_strings_without_corrupting_non_ascii_text() {
    let request = TranslateRequest::from_json(
        r#"{"source_text":"Read café docs.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"}"#,
    )
    .expect("raw UTF-8 JSON should parse");

    assert_eq!(request.source_text.as_deref(), Some("Read café docs."));
}

#[test]
fn serializes_non_ascii_text_without_latin1_corruption() {
    let request = TranslateRequest::direct_text("Read café docs.");
    let json = request.to_json();

    assert!(json.contains("café"));
    assert!(!json.contains("cafÃ"));
}

#[test]
fn parses_common_json_escape_sequences() {
    let request = TranslateRequest::from_json(
        r#"{"source_text":"Read \"docs\"\\now\nplease \u00E9.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"}"#,
    )
    .expect("escaped JSON should parse");

    assert_eq!(
        request.source_text.as_deref(),
        Some("Read \"docs\"\\now\nplease é.")
    );
}

#[test]
fn parses_json_unicode_surrogate_pair() {
    let request = TranslateRequest::from_json(
        r#"{"source_text":"Read \uD83D\uDE00 docs.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"}"#,
    )
    .expect("surrogate pair should parse");

    assert_eq!(request.source_text.as_deref(), Some("Read 😀 docs."));
}
