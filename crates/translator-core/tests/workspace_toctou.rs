#![cfg(unix)]

use std::fs;
use std::os::unix::fs::symlink;

use translator_core::{workspace, ErrorCode};

mod common;
use common::{temp_case, write_file};

#[test]
fn rejects_replaced_validated_target_before_opening_file() {
    let root = temp_case("toctou_after_validation");
    let workspace = root.join("ws");
    let inside = workspace.join("inside.md");
    let outside = root.join("outside.md");
    let link = workspace.join("doc.md");
    fs::create_dir_all(&workspace).expect("workspace");
    write_file(&inside, "Read the docs.");
    write_file(&outside, "Open the file.");
    symlink(&inside, &link).expect("initial inside symlink");

    let err =
        workspace::load_allowed_file_with_open_hook("doc.md", workspace.to_str().unwrap(), || {
            fs::remove_file(&inside).expect("remove validated target");
            symlink(&outside, &inside).expect("replace target with outside symlink");
        })
        .expect_err("replaced validated target should fail");

    assert_eq!(err.code, ErrorCode::PathNotAllowed);
}
