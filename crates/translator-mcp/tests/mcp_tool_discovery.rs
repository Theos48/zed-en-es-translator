mod common;

use serde_json::json;
use translator_mcp::protocol::{TRANSLATE_FILE_TOOL_NAME, TRANSLATE_TEXT_TOOL_NAME};

#[tokio::test]
async fn tool_discovery_lists_exactly_translation_tools() -> Result<(), Box<dyn std::error::Error>>
{
    let mut server = common::spawn_server();
    server.initialize().await?;

    server
        .send_json(&json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list"
        }))
        .await?;

    let response = server.read_response_for_id(2).await?;
    let tools = response["result"]["tools"].as_array().expect("tools array");
    let mut names = tools
        .iter()
        .map(|tool| tool["name"].as_str().expect("tool name"))
        .collect::<Vec<_>>();
    names.sort_unstable();

    assert_eq!(names, [TRANSLATE_FILE_TOOL_NAME, TRANSLATE_TEXT_TOOL_NAME]);
    assert!(tools.iter().all(|tool| tool.get("inputSchema").is_some()));
    Ok(())
}
