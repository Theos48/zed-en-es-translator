mod common;

use std::cell::Cell;
use std::fs;

use serde_json::{json, Value};
use translator_core::{
    ErrorCode, Provider, ProviderConfiguration, ProviderRequest, ProviderResponse,
    ProviderSelection, TranslateFailure,
};
use translator_lsp::state::{ProviderDescriptor, ProviderRuntime};

use common::{file_uri, range, ResponseExt as _, TestClient};

#[test]
fn locality_labels_come_from_the_same_validated_configuration() {
    let offline = ProviderConfiguration::from_values(None, None, None, None).expect("mock");
    let local = ProviderConfiguration::from_values(
        Some("libretranslate"),
        Some("http://127.0.0.1:5000"),
        None,
        None,
    )
    .expect("local");
    let remote = ProviderConfiguration::from_values(
        Some("azure_translator"),
        None,
        Some("AZURE_TRANSLATOR_KEY"),
        Some("true"),
    )
    .expect("remote");

    let labels = [offline, local, remote]
        .iter()
        .map(|configuration| {
            ProviderDescriptor::from_configuration(configuration)
                .action_title()
                .to_string()
        })
        .collect::<Vec<_>>();

    assert_eq!(
        labels,
        [
            "Translate English to Spanish [offline]",
            "Translate English to Spanish [local]",
            "Translate English to Spanish [remote - confirmation required]",
        ]
    );
}

#[test]
fn remote_descriptor_debug_contains_only_safe_state() {
    let configuration = ProviderConfiguration::from_values(
        Some("azure_translator"),
        None,
        Some("PRIVATE_REFERENCE_NAME"),
        Some("true"),
    )
    .expect("remote");

    let debug = format!(
        "{:?}",
        ProviderDescriptor::from_configuration(&configuration)
    );

    assert!(!debug.contains("PRIVATE_REFERENCE_NAME"));
    assert!(!debug.contains("azure"));
    assert!(!debug.contains("http"));
}

#[test]
fn azure_configuration_always_requires_a_fresh_remote_confirmation_label() {
    let configuration = ProviderConfiguration::from_values(
        Some("azure_translator"),
        None,
        Some("AZURE_TRANSLATOR_KEY"),
        Some("true"),
    )
    .expect("remote");

    let descriptor = ProviderDescriptor::from_configuration(&configuration);

    assert!(descriptor.allow_remote());
    assert_eq!(
        descriptor.action_title(),
        "Translate English to Spanish [remote - confirmation required]"
    );
}

#[test]
fn one_configuration_builds_matching_local_selection_and_descriptor() {
    let configuration = ProviderConfiguration::from_values(
        Some("libretranslate"),
        Some("http://127.0.0.1:5000"),
        None,
        None,
    )
    .expect("local");

    let (selection, descriptor) = ProviderRuntime::from_configuration(configuration)
        .expect("runtime")
        .into_parts();

    assert!(matches!(selection, ProviderSelection::LibreTranslate(_)));
    assert_eq!(
        descriptor.action_title(),
        "Translate English to Spanish [local]"
    );
}

#[test]
fn provider_failure_matrix_emits_only_safe_bounded_diagnostics_and_never_mutates_files() {
    for (code, private_detail) in [
        (ErrorCode::ProviderFailed, "PRIVATE_UNAVAILABLE_DETAIL"),
        (ErrorCode::ProviderTimeout, "PRIVATE_TIMEOUT_DETAIL"),
        (ErrorCode::ProviderFailed, "PRIVATE_INVALID_RESPONSE_BODY"),
    ] {
        let workspace = operational_workspace(code.as_str());
        let document = workspace.join("source.md");
        let source = b"SOURCE_MARKER_PRIVATE";
        fs::write(&document, source).expect("source fixture");
        let uri = file_uri(&document);
        let mut client = TestClient::with_provider(
            workspace,
            AlwaysFailingProvider {
                code,
                private_detail,
            },
            ProviderDescriptor::local(),
        );
        client.open(&uri, 1, "markdown", "SOURCE_MARKER_PRIVATE");

        let (response, diagnostics) =
            client.request_with_messages("workspace/executeCommand", execute_params(&uri, 1));
        let error = response.error().expect("normalized provider failure");
        let observed = format!("{error:?} {diagnostics:?}");

        assert!(error.message.contains(code.as_str()));
        assert!(error.message.len() <= 96);
        for prohibited in [
            private_detail,
            "SOURCE_MARKER_PRIVATE",
            document.to_str().expect("path"),
            uri.as_str(),
        ] {
            assert!(!observed.contains(prohibited));
        }
        assert_eq!(fs::read(&document).expect("source after failure"), source);
        client.shutdown();
    }
}

#[test]
fn a_failed_retranslation_invalidates_the_old_preview_without_mutating_the_document() {
    let workspace = operational_workspace("stale-preview");
    let document = workspace.join("source.md");
    let source = b"Read the docs.";
    fs::write(&document, source).expect("source fixture");
    let uri = file_uri(&document);
    let mut client = TestClient::with_provider(
        workspace,
        SucceedThenFailProvider {
            calls: Cell::new(0),
        },
        ProviderDescriptor::local(),
    );
    client.open(&uri, 1, "markdown", "Read the docs.");

    let first = client.request("workspace/executeCommand", execute_params(&uri, 1));
    assert_eq!(first.result(), Some(&Value::Null));
    let preview = client.request(
        "textDocument/hover",
        json!({"textDocument":{"uri":uri},"position":{"line":0,"character":2}}),
    );
    assert!(preview.result().is_some_and(|result| !result.is_null()));

    let second = client.request("workspace/executeCommand", execute_params(&uri, 1));
    assert!(second.error().is_some());
    let stale = client.request(
        "textDocument/hover",
        json!({"textDocument":{"uri":uri},"position":{"line":0,"character":2}}),
    );
    assert_eq!(stale.result(), Some(&Value::Null));
    assert_eq!(fs::read(&document).expect("source after retry"), source);
    client.shutdown();
}

struct AlwaysFailingProvider {
    code: ErrorCode,
    private_detail: &'static str,
}

impl Provider for AlwaysFailingProvider {
    fn translate(&self, _request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        Err(TranslateFailure::new(self.code, self.private_detail))
    }
}

struct SucceedThenFailProvider {
    calls: Cell<usize>,
}

impl Provider for SucceedThenFailProvider {
    fn translate(&self, request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        let calls = self.calls.get();
        self.calls.set(calls + 1);
        if calls == 0 {
            return Ok(ProviderResponse {
                translated_segments: request
                    .segments
                    .iter()
                    .map(|_| "Lee la documentacion.".to_string())
                    .collect(),
            });
        }
        Err(TranslateFailure::new(
            ErrorCode::ProviderFailed,
            "PRIVATE_INVALID_RESPONSE_BODY",
        ))
    }
}

fn execute_params(uri: &str, version: i32) -> Value {
    json!({
        "command":"en-es-translator.translate",
        "arguments":[{
            "uri":uri,
            "version":version,
            "range":range(0, 14),
            "input_kind":"markdown"
        }]
    })
}

fn operational_workspace(case: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "translator-lsp-operational-{case}-{}",
        std::process::id()
    ));
    fs::create_dir_all(&path).expect("operational workspace");
    path
}
