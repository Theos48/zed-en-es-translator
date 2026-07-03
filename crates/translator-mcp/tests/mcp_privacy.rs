mod common;

use translator_mcp::protocol::{TranslateFileParams, TranslateTextParams};
use translator_mcp::tools::TranslatorMcpServer;

#[test]
fn translate_text_error_does_not_leak_source_or_translation() {
    let result = TranslatorMcpServer::new().translate_text(TranslateTextParams {
        source_text: "Read the docs.".to_string(),
        source_language: Some("fr".to_string()),
        target_language: Some("es".to_string()),
        tone: Some("technical_neutral".to_string()),
        preserve_formatting: Some(true),
    });

    let serialized = serde_json::to_string(&result).expect("serialize tool result");

    assert!(!serialized.contains("Read the docs."));
    assert!(!serialized.contains("Lee la documentacion."));
    assert!(!serialized.contains("fr"));
}

#[test]
fn translate_file_error_does_not_leak_paths_or_secrets() {
    let workspace = common::temp_case("privacy_paths");
    let secret_path = workspace.join("secret.md");
    common::write_file(&secret_path, "TOKEN=abc123\nRead the docs.");

    let result = TranslatorMcpServer::new().translate_file(TranslateFileParams {
        workspace_root: workspace.to_string_lossy().into_owned(),
        file_path: "secret.md".to_string(),
        source_language: Some("en".to_string()),
        target_language: Some("es".to_string()),
        tone: Some("technical_neutral".to_string()),
        preserve_formatting: Some(true),
    });

    let serialized = serde_json::to_string(&result).expect("serialize tool result");

    assert!(!serialized.contains(workspace.to_string_lossy().as_ref()));
    assert!(!serialized.contains("secret.md"));
    assert!(!serialized.contains("TOKEN=abc123"));
    assert!(!serialized.contains("Read the docs."));
}
