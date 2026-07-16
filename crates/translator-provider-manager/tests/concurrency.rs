mod common;

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use translator_provider_manager::lifecycle::{ControlledArtifact, Lifecycle};
use translator_provider_manager::locking::{
    ExclusiveLifecycleLock, SharedInferenceLease, SharedStateLock,
};
use translator_provider_manager::manifest::ProviderManifest;

const UPDATED_DIGEST: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";

#[test]
fn lifecycle_operations_are_serialized_and_cleanup_lease_detects_busy_readers() {
    let root = test_root("locks");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("root");
    fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).expect("private root");

    let lifecycle = ExclusiveLifecycleLock::try_acquire(&root).expect("first lifecycle lock");
    let busy = ExclusiveLifecycleLock::try_acquire(&root).expect_err("second lock must be busy");
    assert_eq!(busy.code(), "BUSY");
    let busy = SharedStateLock::try_acquire(&root).expect_err("state read must not race promotion");
    assert_eq!(busy.code(), "BUSY");
    drop(lifecycle);

    let state_reader = SharedStateLock::try_acquire(&root).expect("bounded state reader");
    let busy = ExclusiveLifecycleLock::try_acquire(&root).expect_err("writer must observe reader");
    assert_eq!(busy.code(), "BUSY");
    drop(state_reader);

    let inference = SharedInferenceLease::try_acquire(&root).expect("inference lease");
    let busy = inference
        .try_exclusive_peer()
        .expect_err("exclusive lease must be busy");
    assert_eq!(busy.code(), "BUSY");
}

#[test]
fn inference_lease_remains_available_while_an_update_is_promoted() {
    let root = test_root("inference-during-update");
    let _ = fs::remove_dir_all(&root);
    let runner = common::RUNNER;
    let artifacts = common::fixture_artifacts();
    let sources = artifacts
        .iter()
        .map(|artifact| ControlledArtifact {
            role: artifact.role.to_string(),
            compressed: artifact.compressed.clone(),
        })
        .collect::<Vec<_>>();
    let first = ProviderManifest::from_json(&common::approved_manifest(runner, &artifacts))
        .expect("first manifest");
    let updated = ProviderManifest::from_json(
        &common::approved_manifest(runner, &artifacts)
            .replace(common::MANIFEST_DIGEST, UPDATED_DIGEST),
    )
    .expect("updated manifest");
    let lifecycle = Lifecycle::new(root.clone());
    lifecycle
        .prepare(&first, common::MANIFEST_DIGEST, runner, &sources)
        .expect("first prepare");
    let inference = SharedInferenceLease::try_acquire(&root).expect("active inference");

    lifecycle
        .prepare(&updated, UPDATED_DIGEST, runner, &sources)
        .expect("update while immutable current is leased");

    assert!(inference.try_exclusive_peer().is_err());
}

fn test_root(case: &str) -> PathBuf {
    std::env::current_dir()
        .expect("current directory")
        .join("target/embedded-concurrency-tests")
        .join(format!("{}-{case}", std::process::id()))
}
