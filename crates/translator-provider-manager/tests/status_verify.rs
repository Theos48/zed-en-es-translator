mod common;

use std::fs;
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

fn prepared_root(case: &str) -> PathBuf {
    let root = std::env::current_dir()
        .expect("current directory")
        .join("target/embedded-status-tests")
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
