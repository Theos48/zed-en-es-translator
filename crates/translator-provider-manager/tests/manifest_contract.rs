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
    let json = valid_manifest_json().replacen(
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        1,
    );

    let error = ProviderManifest::from_json(&json).expect_err("digest mismatch must fail");

    assert_eq!(error.code(), "APPROVAL_REQUIRED");
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
