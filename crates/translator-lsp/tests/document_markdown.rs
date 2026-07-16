mod common;

use std::fs;

use serde_json::{json, Value};

use common::{file_uri, range, ResponseExt as _, TestClient};

#[test]
fn document_preview_preserves_all_protected_markdown_regions() {
    let workspace = temp_workspace();
    let path = workspace.join("doc.md");
    fs::write(&path, "Disk content.").expect("disk file");
    let uri = file_uri(&path);
    let source = concat!(
        "---\ntitle: Read\n---\n",
        "Read the docs with `cargo test` and [Open the file.](docs/readme.md).\n",
        "<pre>Read the docs.</pre>\n",
        "```rust\nlet text = \"Read the docs.\";\n```\n"
    );
    let mut client = TestClient::with_workspace(workspace);
    client.open(&uri, 1, "markdown", source);

    let execute = client.request(
        "workspace/executeCommand",
        json!({
            "command":"en-es-translator.translate",
            "arguments":[{"uri":uri,"version":1,"range":range(0,0),"input_kind":"markdown"}]
        }),
    );
    assert_eq!(execute.result(), Some(&Value::Null));
    let hover = client.request(
        "textDocument/hover",
        json!({"textDocument":{"uri":uri},"position":{"line":3,"character":2}}),
    );
    let preview = hover.result().expect("hover")["contents"]["value"]
        .as_str()
        .expect("preview")
        .to_string();
    assert!(preview.contains("title: Read"));
    assert!(preview.contains("`cargo test`"));
    assert!(preview.contains("](docs/readme.md)"));
    assert!(preview.contains("<pre>Read the docs.</pre>"));
    assert!(preview.contains("let text = \"Read the docs.\";"));
    client.shutdown();
}

#[test]
fn protected_only_document_produces_no_preview() {
    let workspace = temp_workspace();
    let path = workspace.join("protected.md");
    fs::write(&path, "Disk content.").expect("disk file");
    let uri = file_uri(&path);
    let mut client = TestClient::with_workspace(workspace);
    client.open(&uri, 1, "markdown", "```rust\nfn main() {}\n```\n");

    let execute = client.request(
        "workspace/executeCommand",
        json!({
            "command":"en-es-translator.translate",
            "arguments":[{"uri":uri,"version":1,"range":range(0,0),"input_kind":"markdown"}]
        }),
    );
    assert!(execute.error().is_some());
    client.shutdown();
}

fn temp_workspace() -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "translator-lsp-document-markdown-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos()
    ));
    fs::create_dir_all(&path).expect("workspace");
    path
}
