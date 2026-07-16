mod common;

use serde_json::{json, Value};

use common::{range, ResponseExt as _, TestClient};

#[test]
fn execute_command_caches_mock_translation_for_hover_without_source_mutation() {
    let mut client = TestClient::new();
    let uri = "file:///workspace/readme.md";
    let source = "Read the docs.";
    client.open(uri, 1, "markdown", source);

    let (execute, messages) = client.request_with_messages(
        "workspace/executeCommand",
        execute_params(uri, 1, range(0, 14), "markdown"),
    );
    assert_eq!(execute.result(), Some(&Value::Null));
    let wire_messages = format!("{messages:?}");
    assert!(wire_messages.contains("Translation preview ready"));
    assert!(!wire_messages.contains(source));
    assert!(!wire_messages.contains("Lee la documentacion"));

    let hover = client.request(
        "textDocument/hover",
        json!({"textDocument":{"uri":uri},"position":{"line":0,"character":2}}),
    );
    let result = hover.result().expect("hover result");
    assert_eq!(result["contents"]["kind"], "markdown");
    assert_eq!(result["contents"]["value"], "Lee la documentacion.");
    assert_eq!(
        result["range"],
        serde_json::to_value(range(0, 14)).expect("range")
    );

    client.shutdown();
}

#[test]
fn plain_text_preview_is_escaped_and_new_preview_replaces_old_range() {
    let mut client = TestClient::new();
    let uri = "file:///workspace/notes.txt";
    client.open(uri, 1, "plaintext", "# Read\nOpen the file.");

    client.request_with_messages(
        "workspace/executeCommand",
        execute_params(uri, 1, range(0, 6), "text"),
    );
    let first = client.request(
        "textDocument/hover",
        json!({"textDocument":{"uri":uri},"position":{"line":0,"character":2}}),
    );
    assert_eq!(
        first.result().expect("first hover")["contents"]["value"],
        "\\# Lee"
    );

    client.request_with_messages(
        "workspace/executeCommand",
        json!({
            "command":"en-es-translator.translate",
            "arguments":[{
                "uri":uri,
                "version":1,
                "range":{"start":{"line":1,"character":0},"end":{"line":1,"character":14}},
                "input_kind":"text"
            }]
        }),
    );
    let old = client.request(
        "textDocument/hover",
        json!({"textDocument":{"uri":uri},"position":{"line":0,"character":2}}),
    );
    assert_eq!(old.result(), Some(&Value::Null));

    client.shutdown();
}

#[test]
fn invalid_incremental_change_still_invalidates_the_previous_preview() {
    let mut client = TestClient::new();
    let uri = "file:///workspace/readme.md";
    client.open(uri, 1, "markdown", "Read the docs.");
    client.request_with_messages(
        "workspace/executeCommand",
        execute_params(uri, 1, range(0, 14), "markdown"),
    );

    client.notify(
        "textDocument/didChange",
        json!({
            "textDocument":{"uri":uri,"version":2},
            "contentChanges":[{
                "range":range(0,4),
                "text":"Open"
            }]
        }),
    );
    let hover = client.request(
        "textDocument/hover",
        json!({"textDocument":{"uri":uri},"position":{"line":0,"character":2}}),
    );
    assert_eq!(hover.result(), Some(&Value::Null));
    client.shutdown();
}

fn execute_params(uri: &str, version: i32, range: lsp_types::Range, input_kind: &str) -> Value {
    json!({
        "command":"en-es-translator.translate",
        "arguments":[{
            "uri":uri,
            "version":version,
            "range":range,
            "input_kind":input_kind
        }]
    })
}
