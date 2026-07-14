mod common;

use std::fs;

use serde_json::{json, Value};
use translator_core::MAX_INPUT_BYTES;
use translator_lsp::state::ProviderDescriptor;

use common::{file_uri, range, ResponseExt as _, TestClient};

#[test]
fn empty_range_translates_current_saved_document_snapshot() {
    let workspace = temp_workspace("snapshot");
    let path = workspace.join("doc.md");
    fs::write(&path, "Disk content.").expect("disk file");
    let uri = file_uri(&path);
    let mut client = TestClient::with_workspace(workspace, ProviderDescriptor::offline());
    client.open(&uri, 3, "markdown", "Read the docs.");

    let execute = client.request_with_messages(
        "workspace/executeCommand",
        execute_params(&uri, 3, "markdown"),
    );
    assert_eq!(execute.0.result(), Some(&Value::Null));

    let hover = client.request(
        "textDocument/hover",
        json!({"textDocument":{"uri":uri},"position":{"line":0,"character":5}}),
    );
    assert_eq!(
        hover.result().expect("hover")["contents"]["value"],
        "Lee la documentacion."
    );
    client.shutdown();
}

#[test]
fn empty_range_rejects_stale_untitled_unsupported_and_oversized_targets() {
    let workspace = temp_workspace("denials");
    let path = workspace.join("doc.txt");
    fs::write(&path, "Disk content.").expect("disk file");
    let uri = file_uri(&path);
    let mut client = TestClient::with_workspace(workspace, ProviderDescriptor::offline());
    client.open(&uri, 2, "plaintext", "Read the docs.");

    for params in [
        execute_params(&uri, 1, "text"),
        execute_params("untitled:Untitled-1", 1, "text"),
    ] {
        let response = client.request("workspace/executeCommand", params);
        assert!(response.error().is_some());
    }

    client.open("file:///workspace/main.rs", 1, "rust", "fn main() {}");
    let unsupported = client.request(
        "workspace/executeCommand",
        execute_params("file:///workspace/main.rs", 1, "text"),
    );
    assert!(unsupported.error().is_some());

    client.change(&uri, 3, &"x".repeat(MAX_INPUT_BYTES + 1));
    let oversized = client.request("workspace/executeCommand", execute_params(&uri, 3, "text"));
    assert!(oversized.error().is_some());
    client.shutdown();
}

fn execute_params(uri: &str, version: i32, input_kind: &str) -> Value {
    json!({
        "command":"en-es-translator.translate",
        "arguments":[{
            "uri":uri,"version":version,"range":range(0,0),"input_kind":input_kind
        }]
    })
}

fn temp_workspace(case: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "translator-lsp-document-preview-{case}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos()
    ));
    fs::create_dir_all(&path).expect("workspace");
    path
}
