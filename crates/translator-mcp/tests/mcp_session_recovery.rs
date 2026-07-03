mod common;

use serde_json::json;
use translator_mcp::protocol::TRANSLATE_TEXT_TOOL_NAME;

#[tokio::test]
async fn valid_call_succeeds_after_invalid_tool_call() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = common::spawn_server();
    server.initialize().await?;

    server
        .send_json(&json!({
            "jsonrpc": "2.0",
            "id": 30,
            "method": "tools/call",
            "params": {
                "name": "unknown_tool",
                "arguments": {}
            }
        }))
        .await?;
    let invalid = server.read_response_for_id(30).await?;
    assert!(invalid.get("error").is_some());

    server
        .send_json(&json!({
            "jsonrpc": "2.0",
            "id": 31,
            "method": "tools/call",
            "params": {
                "name": TRANSLATE_TEXT_TOOL_NAME,
                "arguments": {
                    "source_text": "Read the docs."
                }
            }
        }))
        .await?;
    let valid = server.read_response_for_id(31).await?;

    assert_eq!(
        valid["result"]["structuredContent"]["translated_text"],
        "Lee la documentacion."
    );
    Ok(())
}
