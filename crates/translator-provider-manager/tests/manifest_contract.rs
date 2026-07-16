mod common;

use translator_provider_manager::manifest::ProviderManifest;

fn valid_manifest_json() -> String {
    let runner = common::RUNNER;
    common::approved_manifest(runner, &common::fixture_artifacts())
}

#[test]
fn manifest_should_accept_complete_local_review_when_publication_is_blocked() {
    let manifest = ProviderManifest::from_json(&valid_manifest_json());

    assert!(manifest.is_ok(), "manifest error: {manifest:?}");
}

#[test]
fn updated_fixture_manifest_should_have_stable_derived_digest() {
    assert_eq!(
        common::updated_manifest_digest(),
        "cf070c7f3f5517e98aacdb86d54e1583702781b30990164e0c2a558169839bce"
    );
}

#[test]
fn manifest_should_reject_unknown_fields() {
    let json = valid_manifest_json().replace(
        "\"schema_version\":1,",
        "\"schema_version\":1,\"unexpected\":true,",
    );

    let error = ProviderManifest::from_json(&json).expect_err("unknown field must fail");

    assert_eq!(error.code(), "MANIFEST_INVALID");
}

#[test]
fn manifest_should_reject_automated_local_approval() {
    let json = valid_manifest_json().replace("\"kind\":\"human\"", "\"kind\":\"automated\"");

    let error = ProviderManifest::from_json(&json).expect_err("automated approval must fail");

    assert_eq!(error.code(), "APPROVAL_REQUIRED");
}

#[test]
fn manifest_should_reject_mismatched_approval_digest() {
    let approved_scope = format!(
        "\"scope\":\"local_activation\",\"artifact_set_digest\":\"{}\"",
        common::MANIFEST_DIGEST
    );
    let json = valid_manifest_json().replacen(
        &approved_scope,
        "\"scope\":\"local_activation\",\"artifact_set_digest\":\"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"",
        1,
    );

    let error = ProviderManifest::from_json(&json).expect_err("digest mismatch must fail");

    assert_eq!(error.code(), "APPROVAL_REQUIRED");
}

#[test]
fn manifest_should_reject_artifact_identity_changed_after_approval() {
    let json = valid_manifest_json().replacen(
        "https://example.invalid/0.zst",
        "https://example.invalid/tampered.zst",
        1,
    );

    let error =
        ProviderManifest::from_json(&json).expect_err("approval must bind artifact identity");

    assert_eq!(error.code(), "MANIFEST_INVALID");
}

#[test]
fn manifest_should_reject_wrong_language_platform_architecture_or_compatibility() {
    for (expected, replacement) in [
        ("\"source_language\":\"en\"", "\"source_language\":\"fr\""),
        ("\"target_language\":\"es\"", "\"target_language\":\"de\""),
        (
            "\"platform\":\"linux-x86_64\"",
            "\"platform\":\"linux-aarch64\"",
        ),
        (
            "\"architecture\":\"base-memory\"",
            "\"architecture\":\"unknown\"",
        ),
        (
            "\"runtime_compatibility\":\"bergamot-model-v3-base-memory\"",
            "\"runtime_compatibility\":\"unknown\"",
        ),
    ] {
        let json = valid_manifest_json().replacen(expected, replacement, 1);
        let error = ProviderManifest::from_json(&json).expect_err("compatibility must fail");
        assert_eq!(error.code(), "MANIFEST_INVALID");
    }
}

#[test]
fn manifest_should_reject_incomplete_license_delivery_or_artifact_roles() {
    for (expected, replacement) in [
        (
            "\"spdx_conclusion\":\"MPL-2.0\"",
            "\"spdx_conclusion\":\"\"",
        ),
        (
            "\"delivery_permission\":\"local_acquisition_approved\"",
            "\"delivery_permission\":\"review_required\"",
        ),
        ("\"role\":\"vocabulary\"", "\"role\":\"model\""),
    ] {
        let json = valid_manifest_json().replacen(expected, replacement, 1);
        let error = ProviderManifest::from_json(&json).expect_err("incomplete review must fail");
        assert_eq!(error.code(), "MANIFEST_INVALID");
    }
}

#[test]
fn manifest_should_reject_artifact_totals_outside_declared_budgets() {
    for (expected, replacement) in [
        ("\"transfer_bytes\":67108864", "\"transfer_bytes\":1"),
        (
            "\"active_installed_bytes\":134217728",
            "\"active_installed_bytes\":1",
        ),
        ("\"lifecycle_bytes\":402653184", "\"lifecycle_bytes\":1"),
    ] {
        let json = valid_manifest_json().replacen(expected, replacement, 1);
        let error = ProviderManifest::from_json(&json).expect_err("resource totals must fit");
        assert_eq!(error.code(), "MANIFEST_INVALID");
    }
}
