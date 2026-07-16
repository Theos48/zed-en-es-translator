mod common;

use std::fs;
use std::path::PathBuf;

use translator_provider_manager::error::ManagerError;
use translator_provider_manager::lifecycle::{ControlledArtifact, Lifecycle};
use translator_provider_manager::manifest::ProviderManifest;
use translator_provider_manager::state::InstallationState;

const UPDATED_DIGEST: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";

#[test]
fn valid_update_preserves_previous_and_rollback_swaps_offline() {
    let root = test_root("rollback");
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
    let updated_json = common::approved_manifest(runner, &artifacts)
        .replace(common::MANIFEST_DIGEST, UPDATED_DIGEST);
    let updated = ProviderManifest::from_json(&updated_json).expect("updated manifest");
    let lifecycle = Lifecycle::new(root.clone());
    lifecycle
        .prepare(&first, common::MANIFEST_DIGEST, runner, &sources)
        .expect("first prepare");
    lifecycle
        .prepare(&updated, UPDATED_DIGEST, runner, &sources)
        .expect("update");

    let before = state(&root);
    assert_eq!(
        before.references(),
        (Some(UPDATED_DIGEST), Some(common::MANIFEST_DIGEST), None)
    );

    lifecycle.rollback().expect("offline rollback");

    let after = state(&root);
    assert_eq!(
        after.references(),
        (Some(common::MANIFEST_DIGEST), Some(UPDATED_DIGEST), None)
    );
}

#[test]
fn rollback_without_previous_fails_without_changing_current() {
    let root = test_root("missing-previous");
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
    let manifest = ProviderManifest::from_json(&common::approved_manifest(runner, &artifacts))
        .expect("manifest");
    let lifecycle = Lifecycle::new(root.clone());
    lifecycle
        .prepare(&manifest, common::MANIFEST_DIGEST, runner, &sources)
        .expect("prepare");
    let before = fs::read(root.join("state.json")).expect("before");

    assert!(lifecycle.rollback().is_err());
    assert_eq!(fs::read(root.join("state.json")).expect("after"), before);
}

#[test]
fn post_promotion_verification_failure_restores_previous_current() {
    let root = test_root("post-promotion-failure");
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
    let before = fs::read(root.join("state.json")).expect("state before update");

    let error = lifecycle
        .prepare_with_post_promotion_check(
            &updated,
            UPDATED_DIGEST,
            runner,
            &sources,
            |check_root, promoted| {
                assert_eq!(state(check_root).references().0, Some(promoted));
                Err(ManagerError::IntegrityFailed)
            },
        )
        .expect_err("post-promotion failure must reject update");

    assert_eq!(error.code(), "INTEGRITY_FAILED");
    assert_eq!(
        fs::read(root.join("state.json")).expect("restored state"),
        before
    );
    assert_eq!(state(&root).references().0, Some(common::MANIFEST_DIGEST));
}

fn state(root: &std::path::Path) -> InstallationState {
    InstallationState::from_json(&fs::read_to_string(root.join("state.json")).expect("state file"))
        .expect("valid state")
}

fn test_root(case: &str) -> PathBuf {
    std::env::current_dir()
        .expect("current directory")
        .join("target/embedded-update-tests")
        .join(format!("{}-{case}", std::process::id()))
}
