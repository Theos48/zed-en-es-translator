mod common;

use serde_json::json;
use translator_mcp::protocol::TRANSLATE_TEXT_TOOL_NAME;

#[tokio::test]
async fn translate_text_returns_successful_tool_result() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = common::spawn_server();
    server.initialize().await?;

    server
        .send_json(&json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "tools/call",
            "params": {
                "name": TRANSLATE_TEXT_TOOL_NAME,
                "arguments": {
                    "source_text": "Read the docs.",
                    "source_language": "en",
                    "target_language": "es",
                    "tone": "technical_neutral",
                    "preserve_formatting": true
                }
            }
        }))
        .await?;

    let response = server.read_response_for_id(3).await?;

    assert_eq!(
        response["result"],
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
    Ok(())
}
