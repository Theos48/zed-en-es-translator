#![allow(dead_code)]

use sha2::{Digest as _, Sha256};

pub const MANIFEST_DIGEST: &str =
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
pub const RUNNER: &[u8] = b"#!/bin/sh\nwhile IFS= read -r _; do :; done\nprintf '%s' '{\"wire_version\":1,\"translations\":[\"Comprobacion sintetica publica.\"]}'\n";

pub struct FixtureArtifact {
    pub role: &'static str,
    pub installed_name: &'static str,
    pub installed: Vec<u8>,
    pub compressed: Vec<u8>,
}

pub fn fixture_artifacts() -> Vec<FixtureArtifact> {
    [
        ("model", "model.bergamot", b"controlled model".as_slice()),
        (
            "vocabulary",
            "vocabulary.spm",
            b"controlled vocabulary".as_slice(),
        ),
        (
            "lexical_shortlist",
            "lexical-shortlist.bin",
            b"controlled lexical shortlist".as_slice(),
        ),
    ]
    .into_iter()
    .map(|(role, installed_name, installed)| FixtureArtifact {
        role,
        installed_name,
        installed: installed.to_vec(),
        compressed: zstd::encode_all(installed, 1).expect("compress fixture"),
    })
    .collect()
}

pub fn approved_manifest(runner: &[u8], artifacts: &[FixtureArtifact]) -> String {
    let artifacts_json = artifacts
        .iter()
        .enumerate()
        .map(|(index, artifact)| {
            format!(
                "{{\"role\":\"{}\",\"record_id\":\"fixture-{index}\",\"record_version\":\"1\",\"architecture\":\"base-memory\",\"source_registry\":\"https://example.invalid/registry\",\"attachment_url\":\"https://example.invalid/{index}.zst\",\"compressed_name\":\"{index}.zst\",\"installed_name\":\"{}\",\"compressed_size\":{},\"compressed_sha256\":\"{}\",\"installed_size\":{},\"installed_sha256\":\"{}\",\"runtime_compatibility\":\"bergamot-model-v3-base-memory\",\"spdx_conclusion\":\"MPL-2.0\",\"license_source\":\"https://example.invalid/license\",\"delivery_permission\":\"local_acquisition_approved\"}}",
                artifact.role,
                artifact.installed_name,
                artifact.compressed.len(),
                sha256(&artifact.compressed),
                artifact.installed.len(),
                sha256(&artifact.installed),
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"schema_version\":1,\"profile_id\":\"bergamot-en-es-linux-x86_64-v1\",\"source_language\":\"en\",\"target_language\":\"es\",\"platform\":\"linux-x86_64\",\"review_status\":\"approved\",\"publication_status\":\"blocked\",\"artifact_set_digest\":\"{MANIFEST_DIGEST}\",\"local_approval\":{{\"kind\":\"human\",\"role\":\"project_maintainer\",\"scope\":\"local_activation\",\"artifact_set_digest\":\"{MANIFEST_DIGEST}\",\"evidence_digest\":\"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\",\"reviewed_at\":\"2026-07-15\"}},\"publication_approval\":null,\"runner\":{{\"name\":\"translator-embedded-runtime\",\"wire_version\":1,\"sha256\":\"{}\",\"size\":{},\"installed_name\":\"translator-embedded-runtime\",\"source_repository\":\"https://github.com/mozilla/translations\",\"source_commit\":\"f31423c7c2c6ed8ae57d71a3d19a9db6f156060e\",\"spdx_conclusion\":\"MPL-2.0\",\"license_source\":\"https://example.invalid/runtime-license\",\"delivery_permission\":\"local_acquisition_approved\"}},\"artifacts\":[{artifacts_json}],\"resource_budgets\":{{\"transfer_bytes\":67108864,\"active_installed_bytes\":134217728,\"lifecycle_bytes\":402653184,\"required_free_bytes\":536870912,\"peak_rss_bytes\":1073741824,\"inference_threads\":4,\"cold_readiness_ms\":10000,\"warm_short_p95_ms\":2000,\"warm_mixed_p95_ms\":5000,\"provider_deadline_ms\":15000}}}}",
        sha256(runner),
        runner.len(),
    )
}

pub fn sha256(content: &[u8]) -> String {
    Sha256::digest(content)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
