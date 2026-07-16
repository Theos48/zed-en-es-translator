use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

use crate::error::ManagerError;

const SCHEMA_VERSION: u32 = 1;
const PROFILE_ID: &str = "bergamot-en-es-linux-x86_64-v1";
const DIGEST_DOMAIN: &[u8] = b"translator-provider-manifest-v1\0";

/// Complete reviewed identity for one embedded provider artifact set.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProviderManifest {
    schema_version: u32,
    profile_id: String,
    source_language: String,
    target_language: String,
    platform: String,
    review_status: String,
    publication_status: String,
    artifact_set_digest: String,
    local_approval: Option<ApprovalRecord>,
    publication_approval: Option<ApprovalRecord>,
    runner: RunnerArtifact,
    artifacts: Vec<ModelArtifact>,
    resource_budgets: ResourceBudgets,
}

/// Human approval bound to one exact artifact-set digest and delivery scope.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApprovalRecord {
    kind: String,
    role: String,
    scope: String,
    artifact_set_digest: String,
    evidence_digest: String,
    reviewed_at: String,
}

/// Reviewed native runner identity.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RunnerArtifact {
    name: String,
    wire_version: u32,
    sha256: String,
    size: u64,
    installed_name: String,
    source_repository: String,
    source_commit: String,
    spdx_conclusion: String,
    license_source: String,
    delivery_permission: String,
}

/// Reviewed immutable language artifact identity.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModelArtifact {
    role: String,
    record_id: String,
    record_version: String,
    architecture: String,
    source_registry: String,
    attachment_url: String,
    compressed_name: String,
    installed_name: String,
    compressed_size: u64,
    compressed_sha256: String,
    installed_size: u64,
    installed_sha256: String,
    runtime_compatibility: String,
    spdx_conclusion: String,
    license_source: String,
    delivery_permission: String,
}

/// Mandatory go/no-go budgets stored with the reviewed profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceBudgets {
    transfer_bytes: u64,
    active_installed_bytes: u64,
    lifecycle_bytes: u64,
    required_free_bytes: u64,
    peak_rss_bytes: u64,
    inference_threads: u32,
    cold_readiness_ms: u64,
    warm_short_p95_ms: u64,
    warm_mixed_p95_ms: u64,
    provider_deadline_ms: u64,
}

#[derive(Serialize)]
struct ManifestDigestPayload<'a> {
    schema_version: u32,
    profile_id: &'a str,
    source_language: &'a str,
    target_language: &'a str,
    platform: &'a str,
    review_status: &'a str,
    publication_status: &'a str,
    runner: &'a RunnerArtifact,
    artifacts: &'a [ModelArtifact],
    resource_budgets: ResourceBudgets,
}

impl ProviderManifest {
    /// Parse and validate a strict manifest without exposing parser internals.
    ///
    /// # Errors
    ///
    /// Returns a stable content-free error when the schema, artifact identity,
    /// budgets, or required human approval is invalid.
    pub fn from_json(input: &str) -> Result<Self, ManagerError> {
        let manifest: Self =
            serde_json::from_str(input).map_err(|_| ManagerError::ManifestInvalid)?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Compute the digest a human approval must bind for a strict manifest.
    ///
    /// The input must contain the complete final artifact, license, delivery,
    /// budget and publication fields. Approval records and their digest fields
    /// are deliberately excluded to avoid a self-reference.
    ///
    /// # Errors
    ///
    /// Returns [`ManagerError::ManifestInvalid`] when the JSON does not match
    /// the supported manifest schema or its canonical payload cannot be
    /// serialized.
    pub fn artifact_set_digest_for_review(input: &str) -> Result<String, ManagerError> {
        let manifest: Self =
            serde_json::from_str(input).map_err(|_| ManagerError::ManifestInvalid)?;
        manifest.compute_artifact_set_digest()
    }

    /// Return the exact reviewed artifact-set digest.
    pub fn artifact_set_digest(&self) -> &str {
        &self.artifact_set_digest
    }

    /// Return the fixed product profile identifier.
    pub fn profile_id(&self) -> &str {
        &self.profile_id
    }

    /// Return the model artifacts in their reviewed order.
    pub fn artifacts(&self) -> &[ModelArtifact] {
        &self.artifacts
    }

    /// Return the reviewed runner identity.
    pub fn runner(&self) -> &RunnerArtifact {
        &self.runner
    }

    /// Return the reviewed resource budgets for safe disclosure.
    pub const fn resource_budgets(&self) -> ResourceBudgets {
        self.resource_budgets
    }

    /// Return the publication state without approval details.
    pub fn publication_status(&self) -> &str {
        &self.publication_status
    }

    fn validate(&self) -> Result<(), ManagerError> {
        if self.schema_version != SCHEMA_VERSION
            || self.profile_id != PROFILE_ID
            || self.source_language != "en"
            || self.target_language != "es"
            || self.platform != "linux-x86_64"
            || self.review_status != "approved"
            || !is_sha256(&self.artifact_set_digest)
            || !has_exact_artifact_roles(&self.artifacts)
            || !self.resource_budgets.are_valid()
            || !self.artifact_totals_fit_budgets()
        {
            return Err(ManagerError::ManifestInvalid);
        }
        if self.artifact_set_digest != self.compute_artifact_set_digest()? {
            return Err(ManagerError::ManifestInvalid);
        }

        let local = self
            .local_approval
            .as_ref()
            .ok_or(ManagerError::ApprovalRequired)?;
        if !local.is_valid_for(
            "project_maintainer",
            "local_activation",
            &self.artifact_set_digest,
        ) {
            return Err(ManagerError::ApprovalRequired);
        }

        match self.publication_status.as_str() {
            "blocked" if self.publication_approval.is_none() => {}
            "approved" => {
                let publication = self
                    .publication_approval
                    .as_ref()
                    .ok_or(ManagerError::ApprovalRequired)?;
                if !publication.is_valid_for(
                    "f009_human_reviewer",
                    "publication",
                    &self.artifact_set_digest,
                ) {
                    return Err(ManagerError::ApprovalRequired);
                }
            }
            _ => return Err(ManagerError::ManifestInvalid),
        }

        self.runner.validate()?;
        self.artifacts.iter().try_for_each(ModelArtifact::validate)
    }

    fn compute_artifact_set_digest(&self) -> Result<String, ManagerError> {
        let payload = ManifestDigestPayload {
            schema_version: self.schema_version,
            profile_id: &self.profile_id,
            source_language: &self.source_language,
            target_language: &self.target_language,
            platform: &self.platform,
            review_status: &self.review_status,
            publication_status: &self.publication_status,
            runner: &self.runner,
            artifacts: &self.artifacts,
            resource_budgets: self.resource_budgets,
        };
        let encoded = serde_json::to_vec(&payload).map_err(|_| ManagerError::ManifestInvalid)?;
        let mut digest = Sha256::new();
        digest.update(DIGEST_DOMAIN);
        digest.update(encoded);
        Ok(digest
            .finalize()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect())
    }

    fn artifact_totals_fit_budgets(&self) -> bool {
        let transfer = self.artifacts.iter().try_fold(0_u64, |total, artifact| {
            total.checked_add(artifact.compressed_size)
        });
        let active = self
            .artifacts
            .iter()
            .try_fold(self.runner.size, |total, artifact| {
                total.checked_add(artifact.installed_size)
            });
        let Some((transfer, active)) = transfer.zip(active) else {
            return false;
        };
        let lifecycle = active
            .checked_mul(3)
            .and_then(|total| total.checked_add(transfer));
        transfer <= self.resource_budgets.transfer_bytes
            && active <= self.resource_budgets.active_installed_bytes
            && lifecycle.is_some_and(|total| total <= self.resource_budgets.lifecycle_bytes)
    }
}

impl ApprovalRecord {
    fn is_valid_for(&self, role: &str, scope: &str, digest: &str) -> bool {
        self.kind == "human"
            && self.role == role
            && self.scope == scope
            && self.artifact_set_digest == digest
            && is_sha256(&self.evidence_digest)
            && is_iso_date(&self.reviewed_at)
    }
}

impl RunnerArtifact {
    /// Return the installed runner basename.
    pub fn installed_name(&self) -> &str {
        &self.installed_name
    }

    /// Return the expected runner SHA-256.
    pub fn sha256(&self) -> &str {
        &self.sha256
    }

    /// Return the exact installed runner size.
    pub const fn size(&self) -> u64 {
        self.size
    }

    /// Return a bounded, non-path identity for disclosure.
    pub fn disclosure_identity(&self) -> String {
        format!(
            "{}@{};license={};delivery={}",
            self.name, self.source_commit, self.spdx_conclusion, self.delivery_permission
        )
    }

    fn validate(&self) -> Result<(), ManagerError> {
        if self.name != "translator-embedded-runtime"
            || self.wire_version != 1
            || !is_safe_basename(&self.installed_name)
            || self.installed_name != self.name
            || !is_sha256(&self.sha256)
            || self.size == 0
            || self.source_repository != "https://github.com/mozilla/translations"
            || !is_git_commit(&self.source_commit)
            || self.spdx_conclusion.is_empty()
            || !self.license_source.starts_with("https://")
            || self.delivery_permission != "local_acquisition_approved"
        {
            return Err(ManagerError::ManifestInvalid);
        }
        Ok(())
    }
}

impl ModelArtifact {
    /// Return the artifact role.
    pub fn role(&self) -> &str {
        &self.role
    }

    /// Return the exact reviewed attachment URL.
    pub fn attachment_url(&self) -> &str {
        &self.attachment_url
    }

    /// Return the compressed artifact basename.
    pub fn compressed_name(&self) -> &str {
        &self.compressed_name
    }

    /// Return the installed artifact basename.
    pub fn installed_name(&self) -> &str {
        &self.installed_name
    }

    /// Return the exact compressed size.
    pub const fn compressed_size(&self) -> u64 {
        self.compressed_size
    }

    /// Return the exact expanded size.
    pub const fn installed_size(&self) -> u64 {
        self.installed_size
    }

    /// Return the expected compressed SHA-256.
    pub fn compressed_sha256(&self) -> &str {
        &self.compressed_sha256
    }

    /// Return the expected installed SHA-256.
    pub fn installed_sha256(&self) -> &str {
        &self.installed_sha256
    }

    /// Return a bounded record/version/license identity for disclosure.
    pub fn disclosure_identity(&self) -> String {
        format!(
            "{}={}@{};license={};delivery={}",
            self.role,
            self.record_id,
            self.record_version,
            self.spdx_conclusion,
            self.delivery_permission
        )
    }

    fn validate(&self) -> Result<(), ManagerError> {
        if !matches!(
            self.role.as_str(),
            "model" | "vocabulary" | "lexical_shortlist"
        ) || self.record_id.is_empty()
            || self.record_version.is_empty()
            || self.architecture != "base-memory"
            || !self.source_registry.starts_with("https://")
            || !self.attachment_url.starts_with("https://")
            || !is_safe_basename(&self.compressed_name)
            || !is_safe_basename(&self.installed_name)
            || self.compressed_size == 0
            || self.installed_size == 0
            || !is_sha256(&self.compressed_sha256)
            || !is_sha256(&self.installed_sha256)
            || self.runtime_compatibility != "bergamot-model-v3-base-memory"
            || self.spdx_conclusion.is_empty()
            || !self.license_source.starts_with("https://")
            || self.delivery_permission != "local_acquisition_approved"
        {
            return Err(ManagerError::ManifestInvalid);
        }
        Ok(())
    }
}

impl ResourceBudgets {
    /// Return the maximum reviewed acquisition transfer.
    pub const fn transfer_bytes(self) -> u64 {
        self.transfer_bytes
    }

    /// Return the maximum active installed footprint.
    pub const fn active_installed_bytes(self) -> u64 {
        self.active_installed_bytes
    }

    /// Return the maximum lifecycle footprint.
    pub const fn lifecycle_bytes(self) -> u64 {
        self.lifecycle_bytes
    }

    /// Return the free-space prerequisite checked before root mutation.
    pub const fn required_free_bytes(self) -> u64 {
        self.required_free_bytes
    }

    fn are_valid(self) -> bool {
        self.transfer_bytes > 0
            && self.transfer_bytes <= 64 * 1024 * 1024
            && self.active_installed_bytes > 0
            && self.active_installed_bytes <= 128 * 1024 * 1024
            && self.lifecycle_bytes > 0
            && self.lifecycle_bytes <= 384 * 1024 * 1024
            && self.required_free_bytes == 512 * 1024 * 1024
            && self.peak_rss_bytes <= 1024 * 1024 * 1024
            && self.inference_threads <= 4
            && self.cold_readiness_ms <= 10_000
            && self.warm_short_p95_ms <= 2_000
            && self.warm_mixed_p95_ms <= 5_000
            && self.provider_deadline_ms == 15_000
    }
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn is_git_commit(value: &str) -> bool {
    matches!(value.len(), 40 | 64) && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn has_exact_artifact_roles(artifacts: &[ModelArtifact]) -> bool {
    if artifacts.len() != 3 {
        return false;
    }
    let mut roles = artifacts
        .iter()
        .map(|artifact| artifact.role.as_str())
        .collect::<Vec<_>>();
    roles.sort_unstable();
    roles == ["lexical_shortlist", "model", "vocabulary"]
}

fn is_safe_basename(value: &str) -> bool {
    !value.is_empty()
        && value != "."
        && value != ".."
        && !value.contains('/')
        && !value.contains('\\')
}

fn is_iso_date(value: &str) -> bool {
    value.len() == 10
        && value.as_bytes().get(4) == Some(&b'-')
        && value.as_bytes().get(7) == Some(&b'-')
        && value
            .bytes()
            .enumerate()
            .all(|(index, byte)| matches!(index, 4 | 7) || byte.is_ascii_digit())
}
