use serde_json::Value;
use translator_mcp::protocol::{translate_file_input_schema, translate_text_input_schema};

#[test]
fn translate_text_schema_matches_versioned_contract() {
    let expected: Value = serde_json::from_str(include_str!(
        "../../../specs/002-mcp-server/contracts/translate-text.input.schema.json"
    ))
    .expect("contract schema json");

    assert_eq!(
        translate_text_input_schema(),
        strip_schema_metadata(expected)
    );
}

#[test]
fn translate_file_schema_matches_versioned_contract() {
    let expected: Value = serde_json::from_str(include_str!(
        "../../../specs/002-mcp-server/contracts/translate-file.input.schema.json"
    ))
    .expect("contract schema json");

    assert_eq!(
        translate_file_input_schema(),
        strip_schema_metadata(expected)
    );
}

fn strip_schema_metadata(mut value: Value) -> Value {
    let object = value.as_object_mut().expect("schema object");
    object.remove("$schema");
    object.remove("$id");
    object.remove("title");
    value
}
