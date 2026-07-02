#![cfg(unix)]

use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

use translator_core::{translate_file, ErrorCode, MockProvider};

fn temp_case() -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "zed_translator_toctou_{}_{}",
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
fn rejects_swapped_symlink_before_reading() {
    let root = temp_case();
    let workspace = root.join("ws");
    let inside = workspace.join("inside.md");
    let outside = root.join("outside.md");
    let link = workspace.join("doc.md");
    fs::create_dir_all(&workspace).expect("workspace");
    write_file(&inside, "Read the docs.");
    write_file(&outside, "Read the docs.");
    symlink(&inside, &link).expect("initial inside symlink");
    fs::remove_file(&link).expect("remove inside symlink");
    symlink(&outside, &link).expect("swapped outside symlink");

    let err = translate_file("doc.md", workspace.to_str().unwrap(), &MockProvider::new())
        .expect_err("swapped symlink should fail");

    assert_eq!(err.code, ErrorCode::PathNotAllowed);
}
