use serde_json::Value;
use translator_mcp::protocol::{translate_file_input_schema, translate_text_input_schema};

#[test]
fn translate_text_schema_matches_versioned_contract() {
    let expected: Value = serde_json::from_str(include_str!(
        "../../../specs/002-mcp-server/contracts/translate-text.input.schema.json"
    ))
    .expect("contract schema json");

    assert_eq!(
        strip_schema_metadata_and_additive_fields(translate_text_input_schema()),
        strip_schema_metadata_and_additive_fields(expected)
    );
}

#[test]
fn translate_file_schema_matches_versioned_contract() {
    let expected: Value = serde_json::from_str(include_str!(
        "../../../specs/002-mcp-server/contracts/translate-file.input.schema.json"
    ))
    .expect("contract schema json");

    assert_eq!(
        strip_schema_metadata_and_additive_fields(translate_file_input_schema()),
        strip_schema_metadata_and_additive_fields(expected)
    );
}

fn strip_schema_metadata_and_additive_fields(mut value: Value) -> Value {
    let object = value.as_object_mut().expect("schema object");
    object.remove("$schema");
    object.remove("$id");
    object.remove("title");
    if let Some(properties) = object.get_mut("properties").and_then(Value::as_object_mut) {
        properties.remove("remote_confirmed");
    }
    value
}
