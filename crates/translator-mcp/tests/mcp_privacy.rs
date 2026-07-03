mod common;

use serde_json::json;
use translator_mcp::protocol::{
    TranslateFileParams, TranslateTextParams, TRANSLATE_FILE_TOOL_NAME, TRANSLATE_TEXT_TOOL_NAME,
};
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

#[tokio::test]
async fn stdio_translate_text_error_does_not_leak_source_or_translation(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut server = common::spawn_server();
    server.initialize().await?;

    server
        .send_json(&json!({
            "jsonrpc": "2.0",
            "id": 40,
            "method": "tools/call",
            "params": {
                "name": TRANSLATE_TEXT_TOOL_NAME,
                "arguments": {
                    "source_text": "Read the docs.",
                    "source_language": "fr",
                    "target_language": "es",
                    "tone": "technical_neutral",
                    "preserve_formatting": true
                }
            }
        }))
        .await?;

    let response = server.read_response_for_id(40).await?;
    let stderr = server.drain_stderr().await?;
    let response_text = response.to_string();

    assert_eq!(
        response["result"]["structuredContent"]["code"],
        "UNSUPPORTED_LANGUAGE_PAIR"
    );
    for output in [response_text.as_str(), stderr.as_str()] {
        assert!(!output.contains("Read the docs."));
        assert!(!output.contains("Lee la documentacion."));
        assert!(!output.contains("fr"));
    }
    Ok(())
}

#[tokio::test]
async fn stdio_translate_file_error_does_not_leak_paths_or_secrets(
) -> Result<(), Box<dyn std::error::Error>> {
    let workspace = common::temp_case("stdio_privacy_paths");
    common::write_file(&workspace.join("secret.md"), "TOKEN=abc123\nRead the docs.");
    let mut server = common::spawn_server();
    server.initialize().await?;

    server
        .send_json(&json!({
            "jsonrpc": "2.0",
            "id": 41,
            "method": "tools/call",
            "params": {
                "name": TRANSLATE_FILE_TOOL_NAME,
                "arguments": {
                    "workspace_root": workspace.to_string_lossy(),
                    "file_path": "secret.md",
                    "source_language": "en",
                    "target_language": "es",
                    "tone": "technical_neutral",
                    "preserve_formatting": true
                }
            }
        }))
        .await?;

    let response = server.read_response_for_id(41).await?;
    let stderr = server.drain_stderr().await?;
    let response_text = response.to_string();
    let workspace_text = workspace.to_string_lossy();

    assert_eq!(
        response["result"]["structuredContent"]["code"],
        "PATH_NOT_ALLOWED"
    );
    for output in [response_text.as_str(), stderr.as_str()] {
        assert!(!output.contains(workspace_text.as_ref()));
        assert!(!output.contains("secret.md"));
        assert!(!output.contains("TOKEN=abc123"));
        assert!(!output.contains("Read the docs."));
    }
    Ok(())
}
