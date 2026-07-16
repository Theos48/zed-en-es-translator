mod common;

use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::PathBuf;

use translator_provider_manager::error::ManagerError;
use translator_provider_manager::lifecycle::{ControlledArtifact, Lifecycle, LifecycleCheckpoint};
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
        .update(&updated, UPDATED_DIGEST, runner, &sources)
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
        .update_with_post_promotion_check(
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

#[test]
fn update_should_require_an_existing_current_set() {
    let root = test_root("update-without-current");
    let _ = fs::remove_dir_all(&root);
    let runner = common::RUNNER;
    let artifacts = common::fixture_artifacts();
    let sources = controlled_sources(&artifacts);
    let updated = ProviderManifest::from_json(
        &common::approved_manifest(runner, &artifacts)
            .replace(common::MANIFEST_DIGEST, UPDATED_DIGEST),
    )
    .expect("updated manifest");

    let error = Lifecycle::new(root.clone())
        .update(&updated, UPDATED_DIGEST, runner, &sources)
        .expect_err("update without current must fail");

    assert_eq!(error.code(), "STATE_INVALID");
    assert!(!root.exists());
}

#[test]
fn prepare_should_not_replace_a_different_current_set() {
    let root = test_root("prepare-over-current");
    let _ = fs::remove_dir_all(&root);
    let runner = common::RUNNER;
    let artifacts = common::fixture_artifacts();
    let sources = controlled_sources(&artifacts);
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
    let before = fs::read(root.join("state.json")).expect("state before prepare");

    let error = lifecycle
        .prepare(&updated, UPDATED_DIGEST, runner, &sources)
        .expect_err("prepare must not act as update");

    assert_eq!(error.code(), "STATE_INVALID");
    assert_eq!(
        fs::read(root.join("state.json")).expect("state after"),
        before
    );
}

#[test]
fn interruptions_before_candidate_persistence_should_preserve_current() {
    for checkpoint in [
        LifecycleCheckpoint::StagingCreated,
        LifecycleCheckpoint::ObjectsStaged,
        LifecycleCheckpoint::ObjectsFinalized,
        LifecycleCheckpoint::SetFinalized,
    ] {
        let root = test_root(&format!("interrupted-{checkpoint:?}"));
        let _ = fs::remove_dir_all(&root);
        let (lifecycle, runner, sources, updated) = prepared_update(&root);
        let before = fs::read(root.join("state.json")).expect("state before interruption");

        let error = lifecycle
            .update_with_checkpoint(&updated, UPDATED_DIGEST, runner, &sources, |observed, _| {
                if observed == checkpoint {
                    Err(ManagerError::StorageFailed)
                } else {
                    Ok(())
                }
            })
            .expect_err("injected interruption must fail");

        assert_eq!(error.code(), "STORAGE_FAILED");
        assert_eq!(
            fs::read(root.join("state.json")).expect("state after interruption"),
            before,
            "checkpoint {checkpoint:?} changed current"
        );
    }
}

#[test]
fn interrupted_candidate_should_be_rejected_before_a_successful_retry() {
    let root = test_root("candidate-interruption");
    let _ = fs::remove_dir_all(&root);
    let (lifecycle, runner, sources, updated) = prepared_update(&root);

    let error = lifecycle
        .update_with_checkpoint(
            &updated,
            UPDATED_DIGEST,
            runner,
            &sources,
            |observed, check_root| {
                if observed == LifecycleCheckpoint::CandidatePersisted {
                    assert_eq!(
                        state(check_root).references(),
                        (Some(common::MANIFEST_DIGEST), None, Some(UPDATED_DIGEST))
                    );
                    Err(ManagerError::StorageFailed)
                } else {
                    Ok(())
                }
            },
        )
        .expect_err("candidate interruption must fail");
    assert_eq!(error.code(), "STORAGE_FAILED");
    assert_eq!(
        state(&root).references(),
        (Some(common::MANIFEST_DIGEST), None, Some(UPDATED_DIGEST))
    );

    lifecycle
        .update(&updated, UPDATED_DIGEST, runner, &sources)
        .expect("retry should reject stale candidate and promote cleanly");

    assert_eq!(
        state(&root).references(),
        (Some(UPDATED_DIGEST), Some(common::MANIFEST_DIGEST), None)
    );
}

#[test]
fn update_should_reject_a_symlinked_staging_directory_without_writing_outside() {
    let root = test_root("symlinked-staging");
    let outside = test_root("symlinked-staging-outside");
    let _ = fs::remove_dir_all(&root);
    let _ = fs::remove_dir_all(&outside);
    let (lifecycle, runner, sources, updated) = prepared_update(&root);
    fs::create_dir_all(&outside).expect("outside fixture");
    fs::set_permissions(&outside, fs::Permissions::from_mode(0o700))
        .expect("private outside fixture");
    fs::remove_dir(root.join("staging")).expect("replace empty staging directory");
    symlink(&outside, root.join("staging")).expect("symlinked staging fixture");
    let before = fs::read(root.join("state.json")).expect("state before unsafe update");

    let error = lifecycle
        .update(&updated, UPDATED_DIGEST, runner, &sources)
        .expect_err("symlinked staging must fail closed");

    assert_eq!(error.code(), "STORAGE_UNSAFE");
    assert_eq!(
        fs::read(root.join("state.json")).expect("state after"),
        before
    );
    assert_eq!(
        fs::read_dir(&outside)
            .expect("outside remains readable")
            .count(),
        0
    );
    fs::remove_file(root.join("staging")).expect("remove staging symlink");
    fs::remove_dir(outside).expect("remove outside fixture");
}

fn prepared_update(
    root: &std::path::Path,
) -> (
    Lifecycle,
    &'static [u8],
    Vec<ControlledArtifact>,
    ProviderManifest,
) {
    let runner = common::RUNNER;
    let artifacts = common::fixture_artifacts();
    let sources = controlled_sources(&artifacts);
    let first = ProviderManifest::from_json(&common::approved_manifest(runner, &artifacts))
        .expect("first manifest");
    let updated = ProviderManifest::from_json(
        &common::approved_manifest(runner, &artifacts)
            .replace(common::MANIFEST_DIGEST, UPDATED_DIGEST),
    )
    .expect("updated manifest");
    let lifecycle = Lifecycle::new(root.to_path_buf());
    lifecycle
        .prepare(&first, common::MANIFEST_DIGEST, runner, &sources)
        .expect("first prepare");
    (lifecycle, runner, sources, updated)
}

fn controlled_sources(artifacts: &[common::FixtureArtifact]) -> Vec<ControlledArtifact> {
    artifacts
        .iter()
        .map(|artifact| ControlledArtifact {
            role: artifact.role.to_string(),
            compressed: artifact.compressed.clone(),
        })
        .collect()
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
