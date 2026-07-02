#![cfg(unix)]

use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

use translator_core::{translate_file, ErrorCode, MockProvider};

fn temp_case(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "zed_translator_symlink_{}_{}_{}",
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
fn rejects_direct_file_symlink_escape() {
    let root = temp_case("direct");
    let workspace = root.join("ws");
    let outside = root.join("outside.md");
    fs::create_dir_all(&workspace).expect("workspace");
    write_file(&outside, "Read the docs.");
    symlink(&outside, workspace.join("link.md")).expect("symlink");

    let err = translate_file("link.md", workspace.to_str().unwrap(), &MockProvider::new())
        .expect_err("symlink escape should fail");

    assert_eq!(err.code, ErrorCode::PathNotAllowed);
}

#[test]
fn rejects_directory_symlink_escape() {
    let root = temp_case("directory");
    let workspace = root.join("ws");
    let outside_dir = root.join("outside");
    fs::create_dir_all(&workspace).expect("workspace");
    fs::create_dir_all(&outside_dir).expect("outside");
    write_file(&outside_dir.join("secret.md"), "Read the docs.");
    symlink(&outside_dir, workspace.join("linked")).expect("symlink dir");

    let err = translate_file(
        "linked/secret.md",
        workspace.to_str().unwrap(),
        &MockProvider::new(),
    )
    .expect_err("directory symlink escape should fail");

    assert_eq!(err.code, ErrorCode::PathNotAllowed);
}

#[test]
fn rejects_chained_symlink_escape() {
    let root = temp_case("chained");
    let workspace = root.join("ws");
    let outside = root.join("outside.md");
    fs::create_dir_all(&workspace).expect("workspace");
    write_file(&outside, "Read the docs.");
    symlink(&outside, workspace.join("link1.md")).expect("symlink 1");
    symlink(workspace.join("link1.md"), workspace.join("link2.md")).expect("symlink 2");

    let err = translate_file(
        "link2.md",
        workspace.to_str().unwrap(),
        &MockProvider::new(),
    )
    .expect_err("chained symlink escape should fail");

    assert_eq!(err.code, ErrorCode::PathNotAllowed);
}
