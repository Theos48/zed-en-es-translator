use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::PathBuf;

use translator_provider_manager::storage::StorageRoot;

fn test_root(name: &str) -> PathBuf {
    std::env::current_dir()
        .expect("current directory")
        .join("target/embedded-storage-tests")
        .join(format!("{}-{name}", std::process::id()))
}

#[test]
fn storage_should_accept_private_persistent_directory() {
    let root = test_root("private");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("create private root");
    fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).expect("set mode");

    let result = StorageRoot::validate_existing(&root);

    assert!(result.is_ok(), "storage error: {result:?}");
    fs::remove_dir_all(root).expect("remove private root");
}

#[test]
fn storage_should_reject_group_writable_root() {
    let root = test_root("group-writable");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("create root");
    fs::set_permissions(&root, fs::Permissions::from_mode(0o770)).expect("set mode");

    let error = StorageRoot::validate_existing(&root).expect_err("unsafe mode must fail");

    assert_eq!(error.code(), "STORAGE_UNSAFE");
    fs::remove_dir_all(root).expect("remove unsafe root");
}

#[test]
fn storage_should_reject_symlink_root() {
    let base = test_root("link");
    let target = base.join("target");
    let link = base.join("root");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&target).expect("create target");
    symlink(&target, &link).expect("create link");

    let error = StorageRoot::validate_existing(&link).expect_err("symlink must fail");

    assert_eq!(error.code(), "STORAGE_UNSAFE");
    fs::remove_dir_all(base).expect("remove link root");
}
