use translator_core::{translate_document_snapshot, ErrorCode, MockProvider, MAX_INPUT_BYTES};

mod common;
use common::{temp_case, write_file};

#[test]
fn translates_current_snapshot_after_authorizing_the_disk_file() {
    let workspace = temp_case("snapshot_authorized");
    write_file(&workspace.join("doc.txt"), "Disk text.");

    let success = translate_document_snapshot(
        "doc.txt",
        workspace.to_str().expect("workspace"),
        "Read the docs.",
        &MockProvider::new(),
    )
    .expect("snapshot translation");

    assert_eq!(success.translated_text, "Lee la documentacion.");
    assert_eq!(
        std::fs::read_to_string(workspace.join("doc.txt")).expect("unchanged disk file"),
        "Disk text."
    );
}

#[test]
fn preserves_markdown_protected_regions_from_the_snapshot() {
    let workspace = temp_case("snapshot_markdown");
    write_file(&workspace.join("doc.md"), "Disk text.");
    let snapshot = "Read the docs.\n\n```rust\nlet value = \"Read\";\n```\n";

    let success = translate_document_snapshot(
        "doc.md",
        workspace.to_str().expect("workspace"),
        snapshot,
        &MockProvider::new(),
    )
    .expect("snapshot translation");

    assert!(success.translated_text.contains("Lee la documentacion."));
    assert!(success
        .translated_text
        .contains("```rust\nlet value = \"Read\";\n```"));
}

#[test]
fn rejects_unsupported_sensitive_and_missing_disk_targets() {
    let workspace = temp_case("snapshot_denials");
    write_file(&workspace.join("doc.rs"), "fn main() {}");
    write_file(&workspace.join(".env"), "VALUE=secret");

    for (path, expected) in [
        ("doc.rs", ErrorCode::UnsupportedFileType),
        (".env", ErrorCode::PathNotAllowed),
        ("missing.md", ErrorCode::FileNotFound),
    ] {
        let error = translate_document_snapshot(
            path,
            workspace.to_str().expect("workspace"),
            "Read the docs.",
            &MockProvider::new(),
        )
        .expect_err("target must fail");
        assert_eq!(error.code, expected);
    }
}

#[test]
fn rejects_snapshot_over_the_input_limit() {
    let workspace = temp_case("snapshot_limit");
    write_file(&workspace.join("doc.txt"), "Disk text.");
    let snapshot = "x".repeat(MAX_INPUT_BYTES + 1);

    let error = translate_document_snapshot(
        "doc.txt",
        workspace.to_str().expect("workspace"),
        &snapshot,
        &MockProvider::new(),
    )
    .expect_err("oversized snapshot must fail");

    assert_eq!(error.code, ErrorCode::FileTooLarge);
}

#[cfg(unix)]
#[test]
fn rejects_symlink_escape_even_when_the_snapshot_is_safe() {
    use std::os::unix::fs::symlink;

    let root = temp_case("snapshot_symlink");
    let workspace = root.join("workspace");
    let outside = root.join("outside.md");
    std::fs::create_dir_all(&workspace).expect("workspace");
    write_file(&outside, "Read the docs.");
    symlink(&outside, workspace.join("escape.md")).expect("symlink");

    let error = translate_document_snapshot(
        "escape.md",
        workspace.to_str().expect("workspace"),
        "Read the docs.",
        &MockProvider::new(),
    )
    .expect_err("symlink escape must fail");

    assert_eq!(error.code, ErrorCode::PathNotAllowed);
}
