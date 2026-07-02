use std::path::PathBuf;

use std::cell::RefCell;

use translator_core::{
    translate_file, MockProvider, Provider, ProviderRequest, ProviderResponse, TranslateFailure,
};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates dir")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

#[test]
fn translates_markdown_visible_text_and_preserves_code_regions() {
    let root = workspace_root();

    let success = translate_file(
        "tests/fixtures/markdown/readme.md",
        root.to_str().expect("utf-8 workspace root"),
        &MockProvider::new(),
    )
    .expect("markdown fixture should translate");

    assert!(success.translated_text.contains("Lee la documentacion"));
    assert!(success.translated_text.contains("Abre el archivo."));
    assert!(success.translated_text.contains("](docs/readme.md)"));
    assert!(success
        .translated_text
        .contains("```rust\nlet command = \"Read the docs.\";\n```"));
    assert!(success.translated_text.contains("`cargo test`"));
}

struct RecordingProvider {
    segments: RefCell<Vec<String>>,
}

impl RecordingProvider {
    fn new() -> Self {
        Self {
            segments: RefCell::new(Vec::new()),
        }
    }
}

impl Provider for RecordingProvider {
    fn translate(&self, request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        self.segments.borrow_mut().extend(request.segments.clone());
        Ok(ProviderResponse {
            translated_segments: request.segments.clone(),
        })
    }
}

#[test]
fn does_not_send_markdown_code_regions_to_provider() {
    let root = workspace_root();
    let provider = RecordingProvider::new();

    translate_file(
        "tests/fixtures/markdown/readme.md",
        root.to_str().expect("utf-8 workspace root"),
        &provider,
    )
    .expect("markdown fixture should translate");

    let sent = provider.segments.borrow().join("\n");
    assert!(!sent.contains("let command"));
    assert!(!sent.contains("cargo test"));
}
