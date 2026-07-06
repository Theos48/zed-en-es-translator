mod common;

#[tokio::test]
async fn stdio_server_responds_to_initialize() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = common::spawn_server();
    let response = server.initialize().await?;

    assert_eq!(
        response["result"]["capabilities"]["tools"],
        serde_json::json!({})
    );
    Ok(())
}

#[tokio::test]
async fn stdio_server_responds_to_known_mcp_protocol_versions(
) -> Result<(), Box<dyn std::error::Error>> {
    for protocol_version in ["2024-11-05", "2025-03-26", "2025-06-18"] {
        let mut server = common::spawn_server();
        let response = server.initialize_with_protocol(protocol_version).await?;

        assert_eq!(
            response["result"]["capabilities"]["tools"],
            serde_json::json!({})
        );
    }

    Ok(())
}
