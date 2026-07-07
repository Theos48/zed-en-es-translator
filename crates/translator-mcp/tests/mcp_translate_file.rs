mod common;

use serde_json::{json, Value};
use translator_mcp::protocol::TranslateFileParams;
use translator_mcp::tools::TranslatorMcpServer;

#[test]
fn translate_file_returns_plain_text_success_without_mutating_source() {
    let workspace = common::temp_case("file_success");
    let note = workspace.join("note.txt");
    let original = "Read the docs.";
    common::write_file(&note, original);

    let value = translate_file_value(TranslateFileParams {
        workspace_root: workspace.to_string_lossy().into_owned(),
        file_path: "note.txt".to_string(),
        source_language: Some("en".to_string()),
        target_language: Some("es".to_string()),
        tone: Some("technical_neutral".to_string()),
        preserve_formatting: Some(true),
        remote_confirmed: None,
    });
    let after = std::fs::read_to_string(&note).expect("read source file after translation");

    assert_eq!(after, original);
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

fn translate_file_value(params: TranslateFileParams) -> Value {
    let result = TranslatorMcpServer::new().translate_file(params);
    serde_json::to_value(result).expect("serialize tool result")
}
