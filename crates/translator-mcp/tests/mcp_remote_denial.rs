mod common;

use serde_json::json;
use translator_mcp::protocol::TRANSLATE_TEXT_TOOL_NAME;

#[tokio::test]
async fn translate_text_rejects_provider_selection_fields() -> Result<(), Box<dyn std::error::Error>>
{
    let mut server = common::spawn_server();
    server.initialize().await?;

    server
        .send_json(&json!({
            "jsonrpc": "2.0",
            "id": 20,
            "method": "tools/call",
            "params": {
                "name": TRANSLATE_TEXT_TOOL_NAME,
                "arguments": {
                    "source_text": "Read the docs.",
                    "provider": "remote"
                }
            }
        }))
        .await?;

    let response = server.read_response_for_id(20).await?;

    assert_eq!(response["result"]["isError"], true);
    assert_eq!(
        response["result"]["structuredContent"]["code"],
        "INVALID_INPUT"
    );
    Ok(())
}

#[tokio::test]
async fn translate_text_rejects_remote_confirmation_fields(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut server = common::spawn_server();
    server.initialize().await?;

    server
        .send_json(&json!({
            "jsonrpc": "2.0",
            "id": 21,
            "method": "tools/call",
            "params": {
                "name": TRANSLATE_TEXT_TOOL_NAME,
                "arguments": {
                    "source_text": "Read the docs.",
                    "remote_confirmation": true
                }
            }
        }))
        .await?;

    let response = server.read_response_for_id(21).await?;

    assert_eq!(response["result"]["isError"], true);
    assert_eq!(
        response["result"]["structuredContent"]["code"],
        "INVALID_INPUT"
    );
    Ok(())
}
