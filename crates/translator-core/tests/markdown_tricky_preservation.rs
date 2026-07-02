use std::path::PathBuf;

use translator_core::{translate_file, MockProvider};

mod common;
use common::{temp_case, write_file};

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

#[test]
fn preserves_multiline_inline_code_span() {
    let workspace = temp_case("multiline_code_span");
    write_file(
        &workspace.join("multiline.md"),
        "Read before `do not translate\nRead the docs` after.\n",
    );

    let success = translate_file(
        "multiline.md",
        workspace.to_str().expect("utf-8 workspace root"),
        &MockProvider::new(),
    )
    .expect("multiline code span should translate");

    assert!(success
        .translated_text
        .contains("`do not translate\nRead the docs`"));
}

#[test]
fn resumes_translation_after_unclosed_html_block_blank_line() {
    let workspace = temp_case("html_blank");
    write_file(
        &workspace.join("html.md"),
        "<div>\nRead inside HTML.\n\nRead the docs.\n",
    );

    let success = translate_file(
        "html.md",
        workspace.to_str().expect("utf-8 workspace root"),
        &MockProvider::new(),
    )
    .expect("html block should translate after blank line");

    assert!(success
        .translated_text
        .contains("\n\nLee la documentacion."));
}

#[test]
fn preserves_pre_html_block_across_blank_lines_until_closing_tag() {
    let workspace = temp_case("pre_html_blank");
    write_file(
        &workspace.join("pre.md"),
        "<pre>\nRead inside pre.\n\nRead the docs.\n</pre>\nRead the docs.\n",
    );

    let success = translate_file(
        "pre.md",
        workspace.to_str().expect("utf-8 workspace root"),
        &MockProvider::new(),
    )
    .expect("pre block should remain protected until closing tag");

    assert!(success
        .translated_text
        .contains("\n\nRead the docs.\n</pre>\nLee la documentacion."));
}

#[test]
fn preserves_link_destination_with_nested_parentheses() {
    let workspace = temp_case("link_parens");
    write_file(
        &workspace.join("link.md"),
        "Read [the documentation](docs/readme(v1).md) before changing the code.\n",
    );

    let success = translate_file(
        "link.md",
        workspace.to_str().expect("utf-8 workspace root"),
        &MockProvider::new(),
    )
    .expect("link destination should be preserved");

    assert!(success.translated_text.contains("](docs/readme(v1).md)"));
}
