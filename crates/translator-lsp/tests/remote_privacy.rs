mod common;

use std::cell::Cell;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use lsp_server::{Message, Response};
use serde_json::{json, Value};
use translator_core::{
    contains_obvious_secret, ErrorCode, Provider, ProviderRequest, ProviderResponse,
    TranslateFailure,
};
use translator_lsp::state::ProviderDescriptor;

use common::{range, ResponseExt as _, TestClient};

#[test]
fn remote_not_allowlisted_fails_without_prompt_or_provider_contact() {
    let calls = Arc::new(AtomicUsize::new(0));
    let provider = RemoteGateProvider::new(Arc::clone(&calls));
    let mut client = TestClient::with_provider(
        PathBuf::from("/workspace"),
        provider,
        ProviderDescriptor::remote(false),
    );
    let uri = "file:///workspace/readme.md";
    client.open(uri, 1, "markdown", "Read the docs.");

    let response = client.request(
        "workspace/executeCommand",
        execute_params(uri, 1, range(0, 14)),
    );
    assert!(response
        .error()
        .expect("denial")
        .message
        .contains("PROVIDER_NOT_CONFIGURED"));
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    client.shutdown();
}

#[test]
fn confirmed_secret_is_blocked_before_provider_contact() {
    let calls = Arc::new(AtomicUsize::new(0));
    let provider = RemoteGateProvider::new(Arc::clone(&calls));
    let mut client = TestClient::with_provider(
        PathBuf::from("/workspace"),
        provider,
        ProviderDescriptor::remote(true),
    );
    let uri = "file:///workspace/readme.md";
    let secret = "API_KEY=fake_test_key_123456";
    client.open(uri, 1, "markdown", secret);

    let execute_id = client.begin_request(
        "workspace/executeCommand",
        execute_params(uri, 1, range(0, secret.len() as u32)),
    );
    let confirmation = match client.receive() {
        Message::Request(request) => request,
        other => panic!("expected confirmation, got {other:?}"),
    };
    client.respond(Response::new_ok(
        confirmation.id,
        json!({"title":"Send this request"}),
    ));
    let response = client.receive_response(&execute_id).0;
    assert!(response
        .error()
        .expect("secret denial")
        .message
        .contains("SECRET_DETECTED"));
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    client.shutdown();
}

fn execute_params(uri: &str, version: i32, range: lsp_types::Range) -> Value {
    json!({
        "command":"en-es-translator.translate",
        "arguments":[{"uri":uri,"version":version,"range":range,"input_kind":"markdown"}]
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
                "LEAKED_SECRET_DETAIL",
            ));
        }
        self.calls.fetch_add(1, Ordering::SeqCst);
        Ok(ProviderResponse {
            translated_segments: request.segments.clone(),
        })
    }
}
