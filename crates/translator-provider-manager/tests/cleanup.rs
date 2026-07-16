mod common;

use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

use translator_provider_manager::cleanup::clean;
use translator_provider_manager::lifecycle::{ControlledArtifact, Lifecycle};
use translator_provider_manager::locking::SharedInferenceLease;
use translator_provider_manager::manifest::ProviderManifest;

#[test]
fn cleanup_requires_exact_token_and_refuses_unknown_entries() {
    let root = prepared_root("unknown");
    assert!(clean(&root, "wrong-token").is_err());
    assert!(root.join("state.json").exists());
    fs::write(root.join("unknown-private-file"), b"preserve").expect("unknown entry");

    let error = clean(&root, "remove-embedded-provider-data")
        .expect_err("unknown entry must block cleanup");

    assert_eq!(error.code(), "CLEANUP_REFUSED");
    assert_eq!(
        fs::read(root.join("unknown-private-file")).expect("preserved unknown"),
        b"preserve"
    );
}

#[test]
fn cleanup_is_busy_during_inference_and_then_removes_only_owned_state() {
    let root = prepared_root("lease");
    let lease = SharedInferenceLease::try_acquire(&root).expect("shared inference lease");

    let error = clean(&root, "remove-embedded-provider-data").expect_err("busy cleanup");
    assert_eq!(error.code(), "BUSY");
    assert!(root.exists());
    drop(lease);

    clean(&root, "remove-embedded-provider-data").expect("clean owned state");
    assert!(!root.exists());
}

#[test]
fn cleanup_refuses_symlinks_and_hard_links_without_following_them() {
    let symlink_root = prepared_root("symlink");
    let outside = symlink_root
        .parent()
        .expect("test parent")
        .join(format!("{}-outside", std::process::id()));
    fs::write(&outside, b"outside must survive").expect("outside fixture");
    symlink(&outside, symlink_root.join("staging/unsafe-link")).expect("unsafe symlink");
    let error = clean(&symlink_root, "remove-embedded-provider-data")
        .expect_err("symlink must block cleanup");
    assert_eq!(error.code(), "CLEANUP_REFUSED");
    assert_eq!(
        fs::read(&outside).expect("outside preserved"),
        b"outside must survive"
    );

    let hardlink_root = prepared_root("hardlink");
    let owned_file = fs::read_dir(hardlink_root.join("objects"))
        .expect("objects")
        .next()
        .expect("one object")
        .expect("object")
        .path();
    let owned_file = fs::read_dir(owned_file)
        .expect("object files")
        .next()
        .expect("one file")
        .expect("file")
        .path();
    fs::hard_link(&owned_file, hardlink_root.join("staging/unsafe-hard-link"))
        .expect("unsafe hard link");
    let error = clean(&hardlink_root, "remove-embedded-provider-data")
        .expect_err("hard link must block cleanup");
    assert_eq!(error.code(), "CLEANUP_REFUSED");

    fs::remove_file(outside).expect("remove outside fixture");
}

fn prepared_root(case: &str) -> PathBuf {
    let root = std::env::current_dir()
        .expect("current directory")
        .join("target/embedded-cleanup-tests")
        .join(format!("{}-{case}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let runner = common::RUNNER;
    let artifacts = common::fixture_artifacts();
    let manifest = ProviderManifest::from_json(&common::approved_manifest(runner, &artifacts))
        .expect("manifest");
    let sources = artifacts
        .iter()
        .map(|artifact| ControlledArtifact {
            role: artifact.role.to_string(),
            compressed: artifact.compressed.clone(),
        })
        .collect::<Vec<_>>();
    Lifecycle::new(root.clone())
        .prepare(&manifest, common::MANIFEST_DIGEST, runner, &sources)
        .expect("prepare");
    root
}
