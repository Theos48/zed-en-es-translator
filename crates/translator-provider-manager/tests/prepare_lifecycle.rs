mod common;

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use translator_provider_manager::error::ManagerError;
use translator_provider_manager::lifecycle::{ControlledArtifact, Lifecycle, LifecycleCheckpoint};
use translator_provider_manager::manifest::ProviderManifest;
use translator_provider_manager::state::InstallationState;

#[test]
fn mismatched_consent_has_zero_mutation_and_exact_consent_promotes_atomically() {
    let root = test_root("consent");
    let _ = fs::remove_dir_all(&root);
    let runner = common::RUNNER;
    let artifacts = common::fixture_artifacts();
    let manifest = ProviderManifest::from_json(&common::approved_manifest(runner, &artifacts))
        .expect("approved manifest");
    let lifecycle = Lifecycle::new(root.clone());
    let controlled = artifacts
        .iter()
        .map(|artifact| ControlledArtifact {
            role: artifact.role.to_string(),
            compressed: artifact.compressed.clone(),
        })
        .collect::<Vec<_>>();

    assert!(lifecycle
        .prepare(&manifest, &"f".repeat(64), runner, &controlled)
        .is_err());
    assert!(!root.exists(), "rejected consent must not create the root");

    lifecycle
        .prepare(&manifest, common::MANIFEST_DIGEST, runner, &controlled)
        .expect("controlled prepare");

    let state = fs::read_to_string(root.join("state.json")).expect("promoted state");
    assert!(state.contains(common::MANIFEST_DIGEST));
    assert!(!state.contains("candidate_ready"));
    assert_eq!(
        fs::metadata(&root)
            .expect("root metadata")
            .permissions()
            .mode()
            & 0o777,
        0o700
    );
}

#[test]
fn interrupted_prepare_boundaries_never_create_a_current_reference() {
    for checkpoint in [
        LifecycleCheckpoint::StagingCreated,
        LifecycleCheckpoint::ObjectsStaged,
        LifecycleCheckpoint::ObjectsFinalized,
        LifecycleCheckpoint::SetFinalized,
        LifecycleCheckpoint::CandidatePersisted,
    ] {
        let root = test_root(&format!("interrupted-{checkpoint:?}"));
        let _ = fs::remove_dir_all(&root);
        let runner = common::RUNNER;
        let artifacts = common::fixture_artifacts();
        let manifest = ProviderManifest::from_json(&common::approved_manifest(runner, &artifacts))
            .expect("approved manifest");
        let controlled = artifacts
            .iter()
            .map(|artifact| ControlledArtifact {
                role: artifact.role.to_string(),
                compressed: artifact.compressed.clone(),
            })
            .collect::<Vec<_>>();
        let lifecycle = Lifecycle::new(root.clone());

        let error = lifecycle
            .prepare_with_checkpoint(
                &manifest,
                common::MANIFEST_DIGEST,
                runner,
                &controlled,
                |observed, _| {
                    if observed == checkpoint {
                        Err(ManagerError::StorageFailed)
                    } else {
                        Ok(())
                    }
                },
            )
            .expect_err("injected interruption must fail");

        assert_eq!(error.code(), "STORAGE_FAILED");
        if root.join("state.json").exists() {
            assert_eq!(state(&root).references().0, None);
        }
        lifecycle
            .prepare(&manifest, common::MANIFEST_DIGEST, runner, &controlled)
            .expect("retry should reject any stale candidate and promote");
        assert_eq!(state(&root).references().0, Some(common::MANIFEST_DIGEST));
    }
}

#[test]
fn invalid_candidate_leaves_the_previous_current_unchanged() {
    let root = test_root("invalid-update");
    let _ = fs::remove_dir_all(&root);
    let runner = common::RUNNER;
    let artifacts = common::fixture_artifacts();
    let manifest = ProviderManifest::from_json(&common::approved_manifest(runner, &artifacts))
        .expect("approved manifest");
    let lifecycle = Lifecycle::new(root.clone());
    let mut controlled = artifacts
        .iter()
        .map(|artifact| ControlledArtifact {
            role: artifact.role.to_string(),
            compressed: artifact.compressed.clone(),
        })
        .collect::<Vec<_>>();
    lifecycle
        .prepare(&manifest, common::MANIFEST_DIGEST, runner, &controlled)
        .expect("first prepare");
    let before = fs::read(root.join("state.json")).expect("state before update");
    controlled[0].compressed.push(0);

    assert!(lifecycle
        .prepare(&manifest, common::MANIFEST_DIGEST, runner, &controlled)
        .is_err());
    assert_eq!(
        fs::read(root.join("state.json")).expect("state after failure"),
        before
    );
}

#[test]
fn real_offline_smoke_precedes_first_promotion_and_reuse_is_idempotent() {
    let root = test_root("offline-smoke");
    let _ = fs::remove_dir_all(&root);
    let runner = b"#!/bin/sh\nwhile IFS= read -r _; do :; done\nprintf '%s' '{\"wire_version\":1,\"translations\":[\"Comprobacion sintetica publica.\"]}'\n";
    let artifacts = common::fixture_artifacts();
    let manifest = ProviderManifest::from_json(&common::approved_manifest(runner, &artifacts))
        .expect("approved manifest");
    let controlled = artifacts
        .iter()
        .map(|artifact| ControlledArtifact {
            role: artifact.role.to_string(),
            compressed: artifact.compressed.clone(),
        })
        .collect::<Vec<_>>();
    let lifecycle = Lifecycle::new(root.clone());

    lifecycle
        .prepare_with_offline_smoke(&manifest, common::MANIFEST_DIGEST, runner, &controlled)
        .expect("real process smoke before promotion");
    let before = fs::read(root.join("state.json")).expect("promoted state");
    lifecycle
        .prepare_with_offline_smoke(&manifest, common::MANIFEST_DIGEST, runner, &controlled)
        .expect("idempotent reuse");

    assert_eq!(
        fs::read(root.join("state.json")).expect("reused state"),
        before
    );
}

#[test]
fn failed_offline_smoke_never_creates_an_active_reference() {
    let root = test_root("offline-smoke-failure");
    let _ = fs::remove_dir_all(&root);
    let runner = b"#!/bin/sh\nwhile IFS= read -r _; do :; done\nprintf '%s' '{\"wire_version\":1,\"translations\":[\"This public synthetic check verifies offline translation.\"]}'\n";
    let artifacts = common::fixture_artifacts();
    let manifest = ProviderManifest::from_json(&common::approved_manifest(runner, &artifacts))
        .expect("approved manifest");
    let controlled = artifacts
        .iter()
        .map(|artifact| ControlledArtifact {
            role: artifact.role.to_string(),
            compressed: artifact.compressed.clone(),
        })
        .collect::<Vec<_>>();

    let error = Lifecycle::new(root.clone())
        .prepare_with_offline_smoke(
            &manifest,
            manifest.artifact_set_digest(),
            runner,
            &controlled,
        )
        .expect_err("unchanged smoke output must fail quality gate");

    assert_eq!(error.code(), "INTEGRITY_FAILED");
    assert!(!root.join("state.json").exists());
}

fn test_root(case: &str) -> PathBuf {
    std::env::current_dir()
        .expect("current directory")
        .join("target/embedded-lifecycle-tests")
        .join(format!("{}-{case}", std::process::id()))
}

fn state(root: &std::path::Path) -> InstallationState {
    InstallationState::from_json(&fs::read_to_string(root.join("state.json")).expect("state file"))
        .expect("valid state")
}
