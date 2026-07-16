mod common;

use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

use translator_provider_manager::lifecycle::{ControlledArtifact, Lifecycle};
use translator_provider_manager::manifest::ProviderManifest;
use translator_provider_manager::status::{status, verify};

#[test]
fn status_is_bounded_and_verify_rehashes_current_objects_offline() {
    let root = prepared_root("verified");

    let report = status(&root).expect("status");
    let rendered = report.render();
    assert!(rendered.contains("provider_status=ready"));
    assert!(rendered.contains("profile=bergamot-en-es-linux-x86_64-v1"));
    assert!(rendered.contains("installed_bytes="));
    assert!(rendered.len() < 256);
    assert!(!rendered.contains(root.to_string_lossy().as_ref()));
    verify(&root).expect("offline verify");
}

#[test]
fn verify_fails_closed_after_current_object_corruption() {
    let root = prepared_root("corrupt");
    let object = fs::read_dir(root.join("objects"))
        .expect("objects")
        .next()
        .expect("one object")
        .expect("object entry")
        .path();
    let file = fs::read_dir(object)
        .expect("object files")
        .next()
        .expect("one file")
        .expect("file entry")
        .path();
    fs::write(file, b"corrupt").expect("corrupt fixture");

    let error = verify(&root).expect_err("corruption must fail");

    assert_eq!(error.code(), "INTEGRITY_FAILED");
    assert!(!format!("{error:?}").contains(root.to_string_lossy().as_ref()));
}

#[test]
fn status_rejects_a_symlinked_state_file() {
    let root = prepared_root("symlinked-state");
    let outside = test_root("symlinked-state-outside");
    let _ = fs::remove_file(&outside);
    fs::rename(root.join("state.json"), &outside).expect("move state fixture outside root");
    symlink(&outside, root.join("state.json")).expect("symlinked state fixture");

    let error = status(&root).expect_err("state symlink must fail closed");

    assert_eq!(error.code(), "STORAGE_UNSAFE");
    fs::remove_file(root.join("state.json")).expect("remove state symlink");
    fs::remove_file(outside).expect("remove outside state fixture");
}

#[test]
fn verify_rejects_a_symlinked_object_directory() {
    let root = prepared_root("symlinked-object");
    let object = fs::read_dir(root.join("objects"))
        .expect("objects")
        .next()
        .expect("one object")
        .expect("object entry")
        .path();
    let outside = test_root("symlinked-object-outside");
    let _ = fs::remove_dir_all(&outside);
    fs::rename(&object, &outside).expect("move object fixture outside root");
    symlink(&outside, &object).expect("symlinked object fixture");

    let error = verify(&root).expect_err("object-directory symlink must fail closed");

    assert_eq!(error.code(), "STORAGE_UNSAFE");
    fs::remove_file(object).expect("remove object symlink");
    fs::remove_dir_all(outside).expect("remove outside object fixture");
}

fn prepared_root(case: &str) -> PathBuf {
    let root = test_root(case);
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

fn test_root(case: &str) -> PathBuf {
    std::env::current_dir()
        .expect("current directory")
        .join("target/embedded-status-tests")
        .join(format!("{}-{case}", std::process::id()))
}
