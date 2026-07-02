use std::fs;

use translator_core::{translate_file, ErrorCode, MockProvider};

mod common;
use common::{temp_case, write_file};

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
