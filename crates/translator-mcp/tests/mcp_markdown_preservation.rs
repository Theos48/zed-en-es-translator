mod common;

use translator_mcp::protocol::TranslateFileParams;
use translator_mcp::tools::TranslatorMcpServer;

#[test]
fn translate_file_preserves_markdown_code_links_and_html_blocks() {
    let workspace = common::temp_case("markdown_preservation");
    common::write_file(
        &workspace.join("readme.md"),
        concat!(
            "# Read the docs\n\n",
            "[Read the docs](docs/readme.md)\n\n",
            "<pre>\nRead the docs.\n</pre>\n\n",
            "```rust\nlet command = \"Read the docs.\";\n```\n\n",
            "Open the file.\n",
        ),
    );

    let params = TranslateFileParams {
        workspace_root: workspace.to_string_lossy().into_owned(),
        file_path: "readme.md".to_string(),
        source_language: Some("en".to_string()),
        target_language: Some("es".to_string()),
        tone: Some("technical_neutral".to_string()),
        preserve_formatting: Some(true),
    };
    let value = serde_json::to_value(TranslatorMcpServer::new().translate_file(params))
        .expect("serialize tool result");
    let translated = value["structuredContent"]["translated_text"]
        .as_str()
        .expect("translated text");

    assert!(translated.contains("Lee la documentacion"));
    assert!(translated.contains("](docs/readme.md)"));
    assert!(translated.contains("<pre>\nRead the docs.\n</pre>"));
    assert!(translated.contains("```rust\nlet command = \"Read the docs.\";\n```"));
    assert!(translated.contains("Abre el archivo."));
}
