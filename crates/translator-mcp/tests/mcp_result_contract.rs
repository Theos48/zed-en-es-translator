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
fn result_contract_schema_accepts_success_and_error_shapes() {
    let schema = tool_result_schema();
    let success = serde_json::to_value(success_result(
        TranslateSuccess::new("Lee la documentacion.").expect("valid success result"),
    ))
    .expect("serialize success result");
    let error = serde_json::to_value(error_result(TranslateFailure::new(
        ErrorCode::PathNotAllowed,
        "/tmp/private/secret.md must not leak",
    )))
    .expect("serialize error result");

    validate_tool_result(&schema, &success).expect("success should match schema");
    validate_tool_result(&schema, &error).expect("error should match schema");
}

#[test]
fn result_contract_schema_rejects_extra_content_properties() {
    let schema = tool_result_schema();
    let mut value = serde_json::to_value(success_result(
        TranslateSuccess::new("Lee la documentacion.").expect("valid success result"),
    ))
    .expect("serialize success result");
    value["content"][0]["extra"] = json!(true);

    assert!(validate_tool_result(&schema, &value).is_err());
}

#[test]
fn result_contract_schema_rejects_extra_structured_fields() {
    let schema = tool_result_schema();
    let mut value = serde_json::to_value(error_result(TranslateFailure::new(
        ErrorCode::PathNotAllowed,
        "path should be redacted",
    )))
    .expect("serialize error result");
    value["structuredContent"]["path"] = json!("/tmp/private/secret.md");

    assert!(validate_tool_result(&schema, &value).is_err());
}

#[test]
fn result_contract_schema_rejects_success_with_error_structured_fields() {
    let schema = tool_result_schema();
    let mut value = serde_json::to_value(success_result(
        TranslateSuccess::new("Lee la documentacion.").expect("valid success result"),
    ))
    .expect("serialize success result");
    value["structuredContent"]["code"] = json!("PATH_NOT_ALLOWED");
    value["structuredContent"]["message"] = json!("The requested path is not allowed.");

    assert!(validate_tool_result(&schema, &value).is_err());
}

#[test]
fn structured_content_validator_rejects_ambiguous_one_of_matches() {
    let schema = json!({
        "oneOf": [
            {
                "type": "object",
                "additionalProperties": false,
                "required": ["value"],
                "properties": {
                    "value": { "type": "string" }
                }
            },
            {
                "type": "object",
                "additionalProperties": false,
                "required": ["value"],
                "properties": {
                    "value": { "type": "string" }
                }
            }
        ]
    });
    let value = json!({ "value": "same" });

    let error = validate_structured_content(&schema, &value)
        .expect_err("ambiguous oneOf match should be rejected");

    assert!(error.contains("matched 2 oneOf variants"));
}

fn tool_result_schema() -> Value {
    serde_json::from_str(include_str!(
        "../../../specs/002-mcp-server/contracts/tool-result.schema.json"
    ))
    .expect("tool result schema json")
}

fn validate_tool_result(schema: &Value, value: &Value) -> Result<(), String> {
    let object = value
        .as_object()
        .ok_or_else(|| "tool result must be an object".to_string())?;
    for required in schema["required"]
        .as_array()
        .ok_or_else(|| "schema required must be an array".to_string())?
    {
        let required = required
            .as_str()
            .ok_or_else(|| "schema required entries must be strings".to_string())?;
        if !object.contains_key(required) {
            return Err(format!("missing required property `{required}`"));
        }
    }

    validate_content(&schema["properties"]["content"], &value["content"])?;
    if let Some(structured_content) = value.get("structuredContent") {
        validate_structured_content(
            &schema["properties"]["structuredContent"],
            structured_content,
        )?;
    }
    Ok(())
}

fn validate_content(schema: &Value, value: &Value) -> Result<(), String> {
    let items = value
        .as_array()
        .ok_or_else(|| "content must be an array".to_string())?;
    if items.len() != 1 {
        return Err("content must contain exactly one item".to_string());
    }

    let item_schema = &schema["items"];
    let item = items[0]
        .as_object()
        .ok_or_else(|| "content item must be an object".to_string())?;
    if item.len() != item_schema["properties"].as_object().map_or(0, |p| p.len()) {
        return Err("content item has additional properties".to_string());
    }
    if item.get("type") != Some(&json!("text")) {
        return Err("content item type must be text".to_string());
    }
    if !item.get("text").is_some_and(Value::is_string) {
        return Err("content item text must be a string".to_string());
    }
    Ok(())
}

fn validate_structured_content(schema: &Value, value: &Value) -> Result<(), String> {
    let matches = schema["oneOf"]
        .as_array()
        .ok_or_else(|| "structuredContent oneOf must be an array".to_string())?
        .iter()
        .filter(|candidate| validate_closed_string_object(candidate, value).is_ok())
        .count();

    match matches {
        1 => Ok(()),
        _ => Err(format!(
            "structuredContent matched {matches} oneOf variants"
        )),
    }
}

fn validate_closed_string_object(schema: &Value, value: &Value) -> Result<(), String> {
    let object = value
        .as_object()
        .ok_or_else(|| "structuredContent must be an object".to_string())?;
    let properties = schema["properties"]
        .as_object()
        .ok_or_else(|| "structuredContent schema properties must be an object".to_string())?;
    if object.len() != properties.len() {
        return Err("structuredContent has additional properties".to_string());
    }
    for required in schema["required"]
        .as_array()
        .ok_or_else(|| "structuredContent required must be an array".to_string())?
    {
        let required = required
            .as_str()
            .ok_or_else(|| "structuredContent required entries must be strings".to_string())?;
        if !object.get(required).is_some_and(Value::is_string) {
            return Err(format!("missing string property `{required}`"));
        }
    }
    Ok(())
}
