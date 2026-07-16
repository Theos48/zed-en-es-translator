mod common;

use std::cell::Cell;
use std::path::PathBuf;

use serde_json::json;
use translator_core::{
    ErrorCode, Provider, ProviderRequest, ProviderResponse, TranslateFailure, MAX_OUTPUT_BYTES,
};

use common::{range, ResponseExt as _, TestClient};

#[test]
fn invalid_command_uri_range_and_provider_failures_are_redacted() {
    let secret = "SOURCE_SECRET_123";
    let mut invalid = TestClient::new();
    let uri = "file:///workspace/readme.md";
    invalid.open(uri, 1, "markdown", secret);
    for params in [
        json!({"command":"unknown","arguments":[{"source":secret}]}),
        json!({"command":"en-es-translator.translate","arguments":[{"uri":"untitled:secret","version":1,"range":range(0,1),"input_kind":"markdown","source":secret}]}),
        json!({"command":"en-es-translator.translate","arguments":[{"uri":uri,"version":1,"range":range(0,99),"input_kind":"markdown"}]}),
    ] {
        let response = invalid.request("workspace/executeCommand", params);
        let message = &response.error().expect("invalid error").message;
        assert!(!message.contains(secret));
        assert!(!message.contains(uri));
    }
    invalid.shutdown();

    for code in [
        ErrorCode::ProviderFailed,
        ErrorCode::ProviderTimeout,
        ErrorCode::InternalError,
    ] {
        let provider = FailingProvider {
            code,
            calls: Cell::new(0),
        };
        let mut client = TestClient::with_provider(PathBuf::from("/workspace"), provider);
        client.open(uri, 1, "markdown", "Read the docs.");
        let response = client.request(
            "workspace/executeCommand",
            json!({"command":"en-es-translator.translate","arguments":[{"uri":uri,"version":1,"range":range(0,14),"input_kind":"markdown"}]}),
        );
        let message = &response.error().expect("provider error").message;
        assert!(message.contains(code.as_str()));
        assert!(!message.contains("PRIVATE_PROVIDER_DETAIL"));
        assert!(!message.contains(uri));
        client.shutdown();
    }

    for output in [Vec::new(), vec!["x".repeat(MAX_OUTPUT_BYTES + 1)]] {
        let mut client =
            TestClient::with_provider(PathBuf::from("/workspace"), ShapeProvider { output });
        client.open(uri, 1, "markdown", "Read the docs.");
        let response = client.request(
            "workspace/executeCommand",
            json!({"command":"en-es-translator.translate","arguments":[{"uri":uri,"version":1,"range":range(0,14),"input_kind":"markdown"}]}),
        );
        let message = &response.error().expect("invalid output error").message;
        assert!(message.contains("PROVIDER_FAILED"));
        assert!(!message.contains("xxxxx"));
        client.shutdown();
    }
}

struct ShapeProvider {
    output: Vec<String>,
}

impl Provider for ShapeProvider {
    fn translate(&self, _request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        Ok(ProviderResponse {
            translated_segments: self.output.clone(),
        })
    }
}

struct FailingProvider {
    code: ErrorCode,
    calls: Cell<usize>,
}

impl Provider for FailingProvider {
    fn translate(&self, _request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        self.calls.set(self.calls.get() + 1);
        Err(TranslateFailure::new(self.code, "PRIVATE_PROVIDER_DETAIL"))
    }
}
