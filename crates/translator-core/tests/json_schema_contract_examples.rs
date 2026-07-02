use translator_core::{ErrorCode, TranslateRequest};

const REQUEST_SCHEMA: &str = include_str!(
    "../../../specs/001-translation-core-contract/contracts/translate-request.schema.json"
);
const RESULT_SCHEMA: &str = include_str!(
    "../../../specs/001-translation-core-contract/contracts/translate-result.schema.json"
);

#[test]
fn request_schema_defines_exclusive_direct_text_and_file_variants() {
    assert!(REQUEST_SCHEMA.contains(r#""oneOf""#));
    assert!(REQUEST_SCHEMA.contains(r#""DirectTextRequest""#));
    assert!(REQUEST_SCHEMA.contains(r#""FileRequest""#));
    assert!(REQUEST_SCHEMA.contains(r#""source_text""#));
    assert!(REQUEST_SCHEMA.contains(r#""file_path""#));
    assert!(REQUEST_SCHEMA.contains(r#""workspace_root""#));
    assert!(REQUEST_SCHEMA.contains(r#""maxLength": 20480"#));
}

#[test]
fn result_schema_matches_success_and_failure_contract() {
    assert!(RESULT_SCHEMA.contains(r#""translated_text""#));
    assert!(RESULT_SCHEMA.contains(r#""maxLength": 40960"#));
    assert!(RESULT_SCHEMA.contains(r#""code""#));
    assert!(RESULT_SCHEMA.contains(r#""message""#));
    assert!(RESULT_SCHEMA.contains(r#""maxLength": 512"#));
}

#[test]
fn request_parser_rejects_unknown_fields() {
    let err = TranslateRequest::from_json(
        r#"{"source_text":"Read.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"text","provider":"remote"}"#,
    )
    .expect_err("provider config must be rejected");

    assert_eq!(err.code, ErrorCode::InvalidInput);
}

#[test]
fn request_parser_rejects_mixed_direct_text_and_file_context() {
    let err = TranslateRequest::from_json(
        r#"{"source_text":"Read.","source_language":"en","target_language":"es","tone":"technical_neutral","preserve_formatting":true,"input_kind":"markdown","file_path":"README.md","workspace_root":"/workspace"}"#,
    )
    .expect_err("source_text and file context are exclusive");

    assert_eq!(err.code, ErrorCode::InvalidInput);
}
