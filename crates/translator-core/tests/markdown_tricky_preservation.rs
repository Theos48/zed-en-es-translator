use std::path::PathBuf;

use translator_core::{translate_file, MockProvider};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates dir")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

#[test]
fn preserves_tricky_markdown_regions() {
    let root = workspace_root();

    let success = translate_file(
        "tests/fixtures/markdown/tricky_code_regions.md",
        root.to_str().expect("utf-8 workspace root"),
        &MockProvider::new(),
    )
    .expect("tricky markdown fixture should translate");

    assert!(success
        .translated_text
        .contains("---\ntitle: Read the docs"));
    assert!(success
        .translated_text
        .contains("<div>\nRead the docs inside HTML.\n</div>"));
    assert!(success.translated_text.contains("````markdown\n```rust"));
    assert!(success
        .translated_text
        .contains("Use ``cargo test`` antes de cambiando el codigo."));
    assert!(success
        .translated_text
        .ends_with("echo \"Read the docs\"\n"));
}
