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
