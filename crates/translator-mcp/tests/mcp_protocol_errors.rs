mod common;

use serde_json::json;

#[tokio::test]
async fn unknown_tool_returns_protocol_error() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = common::spawn_server();
    server.initialize().await?;

    server
        .send_json(&json!({
            "jsonrpc": "2.0",
            "id": 10,
            "method": "tools/call",
            "params": {
                "name": "translate_everything",
                "arguments": {}
            }
        }))
        .await?;

    let response = server.read_response_for_id(10).await?;

    assert!(response.get("error").is_some());
    assert!(response.get("result").is_none());
    Ok(())
}

#[tokio::test]
async fn malformed_tools_call_shape_returns_protocol_error(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut server = common::spawn_server();
    server.initialize().await?;

    server
        .send_json(&json!({
            "jsonrpc": "2.0",
            "id": 11,
            "method": "tools/call",
            "params": {
                "name": 42,
                "arguments": []
            }
        }))
        .await?;

    let response = server.read_response_for_id(11).await?;

    assert!(response.get("error").is_some());
    assert!(response.get("result").is_none());
    Ok(())
}
