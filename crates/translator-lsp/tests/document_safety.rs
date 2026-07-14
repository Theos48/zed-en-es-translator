mod common;

use std::cell::Cell;
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use serde_json::json;
use translator_core::{Provider, ProviderRequest, ProviderResponse, TranslateFailure};
use translator_lsp::state::ProviderDescriptor;

use common::{file_uri, range, ResponseExt as _, TestClient};

#[test]
fn unsafe_document_targets_fail_before_provider_contact() {
    let root = temp_root();
    let workspace = root.join("workspace");
    fs::create_dir_all(&workspace).expect("workspace");
    let outside = root.join("outside.md");
    fs::write(&outside, "Read the docs.").expect("outside");
    fs::write(workspace.join("credentials.md"), "Read the docs.").expect("sensitive");
    fs::write(workspace.join("data.json"), "{}").expect("unsupported");
    fs::write(workspace.join("binary.md"), [0_u8, 1, 2]).expect("binary");
    fs::write(workspace.join("bad.md"), [0xff_u8, 0xfe]).expect("non utf8");

    #[cfg(unix)]
    std::os::unix::fs::symlink(&outside, workspace.join("escape.md")).expect("symlink");

    let calls = Arc::new(AtomicUsize::new(0));
    let provider = CountingProvider {
        calls: Arc::clone(&calls),
        _not_sync: Cell::new(0),
    };
    let mut client =
        TestClient::with_provider(workspace.clone(), provider, ProviderDescriptor::local());

    let mut paths = vec![
        outside,
        workspace.join("../outside.md"),
        workspace.join("credentials.md"),
        workspace.join("missing.md"),
        workspace.join("data.json"),
        workspace.join("binary.md"),
        workspace.join("bad.md"),
    ];
    #[cfg(unix)]
    paths.push(workspace.join("escape.md"));

    for (index, path) in paths.iter().enumerate() {
        let uri = file_uri(path);
        client.open(&uri, index as i32 + 1, "markdown", "Read the docs.");
        let response = client.request(
            "workspace/executeCommand",
            json!({
                "command":"en-es-translator.translate",
                "arguments":[{"uri":uri,"version":index as i32 + 1,"range":range(0,0),"input_kind":"markdown"}]
            }),
        );
        assert!(response.error().is_some(), "unsafe target should fail");
    }

    assert_eq!(calls.load(Ordering::SeqCst), 0);
    client.shutdown();
}

struct CountingProvider {
    calls: Arc<AtomicUsize>,
    _not_sync: Cell<usize>,
}

impl Provider for CountingProvider {
    fn translate(&self, request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Ok(ProviderResponse {
            translated_segments: request.segments.clone(),
        })
    }
}

fn temp_root() -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "translator-lsp-document-safety-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos()
    ));
    fs::create_dir_all(&path).expect("root");
    path
}
