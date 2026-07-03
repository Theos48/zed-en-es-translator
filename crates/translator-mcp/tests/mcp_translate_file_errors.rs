mod common;

use serde_json::Value;
use translator_mcp::protocol::TranslateFileParams;
use translator_mcp::tools::TranslatorMcpServer;

#[test]
fn translate_file_rejects_unsupported_extension() {
    let workspace = common::temp_case("unsupported_extension");
    common::write_file(&workspace.join("data.json"), "{}");

    let value = translate_file_error_value(params(&workspace, "data.json"));

    assert_error_code(&value, "UNSUPPORTED_FILE_TYPE");
}

#[test]
fn translate_file_rejects_protected_only_markdown() {
    let workspace = common::temp_case("protected_only_markdown");
    common::write_file(&workspace.join("code.md"), "```rust\nfn main() {}\n```\n");

    let value = translate_file_error_value(params(&workspace, "code.md"));

    assert_error_code(&value, "NO_TRANSLATABLE_SEGMENTS");
}

fn params(workspace: &std::path::Path, file_path: &str) -> TranslateFileParams {
    TranslateFileParams {
        workspace_root: workspace.to_string_lossy().into_owned(),
        file_path: file_path.to_string(),
        source_language: Some("en".to_string()),
        target_language: Some("es".to_string()),
        tone: Some("technical_neutral".to_string()),
        preserve_formatting: Some(true),
    }
}

fn translate_file_error_value(params: TranslateFileParams) -> Value {
    let result = TranslatorMcpServer::new().translate_file(params);
    serde_json::to_value(result).expect("serialize tool result")
}

fn assert_error_code(value: &Value, code: &str) {
    assert_eq!(value["isError"], true);
    assert_eq!(value["structuredContent"]["code"], code);
}
