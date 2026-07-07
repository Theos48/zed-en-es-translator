use translator_core::TranslateRequest;

#[test]
fn parses_missing_remote_confirmation_as_false() {
    let request = TranslateRequest::from_json(
        r#"{"source_text":"Read the docs.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text"}"#,
    )
    .expect("request should parse");

    assert!(!request.remote_confirmed);
}

#[test]
fn parses_additive_remote_confirmation() {
    let request = TranslateRequest::from_json(
        r#"{"source_text":"Read the docs.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text","remote_confirmed":true}"#,
    )
    .expect("request should parse");

    assert!(request.remote_confirmed);
}

#[test]
fn serializes_remote_confirmation_only_when_true() {
    let mut request = TranslateRequest::direct_text("Read the docs.");
    assert!(!request.to_json().contains("remote_confirmed"));

    request.remote_confirmed = true;

    assert!(request.to_json().contains(r#""remote_confirmed":true"#));
}
