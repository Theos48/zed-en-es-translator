use std::fs;
use std::path::{Path, PathBuf};

use translator_core::{translate_file, ErrorCode, MockProvider};

fn temp_case(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "zed_translator_{}_{}_{}",
        name,
        std::process::id(),
        unique_suffix()
    ));
    fs::create_dir_all(&root).expect("temp root");
    root
}

fn unique_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time")
        .as_nanos()
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent dir");
    }
    fs::write(path, content).expect("write file");
}

#[test]
fn rejects_parent_directory_traversal() {
    let root = temp_case("traversal");
    let workspace = root.join("ws");
    let outside = root.join("outside.md");
    fs::create_dir_all(&workspace).expect("workspace");
    write_file(&outside, "Read the docs.");

    let err = translate_file(
        "../outside.md",
        workspace.to_str().unwrap(),
        &MockProvider::new(),
    )
    .expect_err("traversal should fail");

    assert_eq!(err.code, ErrorCode::PathNotAllowed);
}

#[test]
fn rejects_absolute_path_outside_workspace() {
    let root = temp_case("absolute");
    let workspace = root.join("ws");
    let outside = root.join("outside.md");
    fs::create_dir_all(&workspace).expect("workspace");
    write_file(&outside, "Read the docs.");

    let err = translate_file(
        outside.to_str().unwrap(),
        workspace.to_str().unwrap(),
        &MockProvider::new(),
    )
    .expect_err("absolute outside path should fail");

    assert_eq!(err.code, ErrorCode::PathNotAllowed);
}

#[test]
fn rejects_root_prefix_confusion() {
    let root = temp_case("prefix");
    let workspace = root.join("ws");
    let evil = root.join("ws-evil");
    fs::create_dir_all(&workspace).expect("workspace");
    fs::create_dir_all(&evil).expect("evil");
    let evil_file = evil.join("secret.md");
    write_file(&evil_file, "Read the docs.");

    let err = translate_file(
        evil_file.to_str().unwrap(),
        workspace.to_str().unwrap(),
        &MockProvider::new(),
    )
    .expect_err("prefix confusion should fail");

    assert_eq!(err.code, ErrorCode::PathNotAllowed);
}
