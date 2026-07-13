mod common;

use std::cell::Cell;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use lsp_server::{Message, RequestId, Response};
use serde_json::{json, Value};
use translator_core::{
    contains_obvious_secret, ErrorCode, Provider, ProviderRequest, ProviderResponse,
    TranslateFailure,
};
use translator_lsp::state::ProviderDescriptor;

use common::{range, TestClient};

#[test]
fn remote_confirmation_is_correlated_default_deny_and_per_request() {
    let calls = Arc::new(AtomicUsize::new(0));
    let provider = RemoteGateProvider::new(Arc::clone(&calls));
    let mut client = TestClient::with_provider(
        PathBuf::from("/workspace"),
        provider,
        ProviderDescriptor::remote(true),
    );
    let uri = "file:///workspace/readme.md";
    client.open(uri, 1, "markdown", "Read the docs.");

    let execute_id = client.begin_request("workspace/executeCommand", execute_params(uri, 1));
    let confirmation = receive_confirmation(&client);
    assert_eq!(confirmation.method, "window/showMessageRequest");
    assert_eq!(
        confirmation.params["actions"]
            .as_array()
            .expect("actions")
            .len(),
        1
    );
    let confirmation_wire = confirmation.params.to_string();
    assert!(!confirmation_wire.contains(uri));
    assert!(!confirmation_wire.contains("Read the docs"));

    client.respond(Response::new_ok(
        RequestId::from("mismatched".to_string()),
        json!({"title":"Send this request"}),
    ));
    client.respond(Response::new_ok(confirmation.id, Value::Null));
    let denied = client.receive_response(&execute_id).0;
    assert!(denied
        .error
        .expect("denial")
        .message
        .contains("REMOTE_CONFIRMATION_REQUIRED"));
    assert_eq!(calls.load(Ordering::SeqCst), 0);

    let second_id = client.begin_request("workspace/executeCommand", execute_params(uri, 1));
    let second_confirmation = receive_confirmation(&client);
    client.respond(Response::new_ok(
        second_confirmation.id,
        json!({"title":"Send this request"}),
    ));
    let accepted = client.receive_response(&second_id).0;
    assert_eq!(accepted.result, Some(Value::Null));
    assert_eq!(calls.load(Ordering::SeqCst), 1);

    let third_id = client.begin_request("workspace/executeCommand", execute_params(uri, 1));
    let third_confirmation = receive_confirmation(&client);
    client.respond(Response::new_ok(third_confirmation.id, Value::Null));
    assert!(client.receive_response(&third_id).0.error.is_some());
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    client.shutdown();
}

#[test]
fn document_change_while_confirming_invalidates_the_request() {
    let calls = Arc::new(AtomicUsize::new(0));
    let provider = RemoteGateProvider::new(Arc::clone(&calls));
    let mut client = TestClient::with_provider(
        PathBuf::from("/workspace"),
        provider,
        ProviderDescriptor::remote(true),
    );
    let uri = "file:///workspace/readme.md";
    client.open(uri, 1, "markdown", "Read the docs.");

    let execute_id = client.begin_request("workspace/executeCommand", execute_params(uri, 1));
    let confirmation = receive_confirmation(&client);
    client.change(uri, 2, "Open the file.");
    client.respond(Response::new_ok(
        confirmation.id,
        json!({"title":"Send this request"}),
    ));
    let response = client.receive_response(&execute_id).0;
    assert!(response.error.is_some());
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    client.shutdown();
}

#[test]
fn cancellation_while_confirming_denies_without_provider_contact() {
    let calls = Arc::new(AtomicUsize::new(0));
    let provider = RemoteGateProvider::new(Arc::clone(&calls));
    let mut client = TestClient::with_provider(
        PathBuf::from("/workspace"),
        provider,
        ProviderDescriptor::remote(true),
    );
    let uri = "file:///workspace/readme.md";
    client.open(uri, 1, "markdown", "Read the docs.");

    let execute_id = client.begin_request("workspace/executeCommand", execute_params(uri, 1));
    let _confirmation = receive_confirmation(&client);
    client.notify("$/cancelRequest", json!({"id":execute_id}));

    let response = client.receive_response(&execute_id).0;
    assert!(response
        .error
        .expect("cancellation denial")
        .message
        .contains("REMOTE_CONFIRMATION_REQUIRED"));
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    client.shutdown();
}

fn receive_confirmation(client: &TestClient) -> lsp_server::Request {
    match client.receive() {
        Message::Request(request) => request,
        other => panic!("expected confirmation request, got {other:?}"),
    }
}

fn execute_params(uri: &str, version: i32) -> Value {
    json!({
        "command":"en-es-translator.translate",
        "arguments":[{"uri":uri,"version":version,"range":range(0,14),"input_kind":"markdown"}]
    })
}

struct RemoteGateProvider {
    calls: Arc<AtomicUsize>,
    _not_sync: Cell<usize>,
}

impl RemoteGateProvider {
    fn new(calls: Arc<AtomicUsize>) -> Self {
        Self {
            calls,
            _not_sync: Cell::new(0),
        }
    }
}

impl Provider for RemoteGateProvider {
    fn translate(&self, request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        if request
            .segments
            .iter()
            .any(|segment| contains_obvious_secret(segment))
        {
            return Err(TranslateFailure::new(
                ErrorCode::SecretDetected,
                "secret detail",
            ));
        }
        if !request.remote_confirmed {
            return Err(TranslateFailure::new(
                ErrorCode::RemoteConfirmationRequired,
                "confirmation detail",
            ));
        }
        self.calls.fetch_add(1, Ordering::SeqCst);
        Ok(ProviderResponse {
            translated_segments: request.segments.clone(),
        })
    }
}
