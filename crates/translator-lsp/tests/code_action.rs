mod common;

use serde_json::Value;
use translator_lsp::state::ProviderDescriptor;

use common::{code_action_params, range, TestClient};

#[test]
fn returns_source_free_non_editing_refactor_action_for_current_selection() {
    let mut client = TestClient::new(ProviderDescriptor::offline());
    let uri = "file:///workspace/readme.md";
    let source = "Read the docs.";
    client.open(uri, 7, "markdown", source);

    let response = client.request(
        "textDocument/codeAction",
        code_action_params(uri, range(0, 14)),
    );
    let actions = response.result.expect("result");
    let action = &actions.as_array().expect("action list")[0];

    assert_eq!(action["title"], "Translate English to Spanish [offline]");
    assert_eq!(action["kind"], "refactor");
    assert!(action.get("edit").is_none());
    assert_eq!(action["command"]["command"], "en-es-translator.translate");
    assert_eq!(action["command"]["arguments"][0]["version"], 7);
    assert_eq!(action["command"]["arguments"][0]["input_kind"], "markdown");
    assert!(!action.to_string().contains(source));
    assert!(action["command"]["arguments"][0]
        .get("source_text")
        .is_none());

    client.shutdown();
}

#[test]
fn returns_no_action_for_unsupported_language_snapshot() {
    let mut client = TestClient::new(ProviderDescriptor::offline());
    let uri = "file:///workspace/main.rs";
    client.open(uri, 1, "rust", "fn main() {}");

    let response = client.request(
        "textDocument/codeAction",
        code_action_params(uri, range(0, 2)),
    );
    assert_eq!(response.result, Some(Value::Array(Vec::new())));

    client.shutdown();
}
