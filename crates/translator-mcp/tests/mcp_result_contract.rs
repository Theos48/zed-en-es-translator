use serde_json::{json, Value};
use translator_core::{ErrorCode, TranslateFailure, TranslateSuccess};
use translator_mcp::protocol::{error_result, success_result};

#[test]
fn success_result_contains_visible_and_structured_translation_only() {
    let result = success_result(
        TranslateSuccess::new("Lee la documentacion.").expect("valid success result"),
    );
    let value = serde_json::to_value(result).expect("serialize result");

    assert_eq!(
        value,
        json!({
            "content": [
                {
                    "type": "text",
                    "text": "Lee la documentacion."
                }
            ],
            "structuredContent": {
                "translated_text": "Lee la documentacion."
            },
            "isError": false
        })
    );
}

#[test]
fn error_result_contains_code_message_and_is_error_true() {
    let result = error_result(TranslateFailure::new(
        ErrorCode::PathNotAllowed,
        "/tmp/private/secret.md must not leak",
    ));
    let value = serde_json::to_value(result).expect("serialize result");

    assert_eq!(
        value,
        json!({
            "content": [
                {
                    "type": "text",
                    "text": "PATH_NOT_ALLOWED: The requested path is not allowed."
                }
            ],
            "structuredContent": {
                "code": "PATH_NOT_ALLOWED",
                "message": "The requested path is not allowed."
            },
            "isError": true
        })
    );
}

#[test]
fn result_contract_schema_accepts_success_shape() {
    let schema: Value = serde_json::from_str(include_str!(
        "../../../specs/002-mcp-server/contracts/tool-result.schema.json"
    ))
    .expect("tool result schema json");

    assert_eq!(schema["required"], json!(["content"]));
}
