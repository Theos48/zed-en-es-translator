use crate::error::ManagerError;
use crate::manifest::ProviderManifest;

/// Bounded, content-free acquisition disclosure for one reviewed manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Disclosure {
    manifest_digest: String,
    transfer_bytes: u64,
    active_bytes: u64,
    lifecycle_bytes: u64,
    publication: String,
    runner_identity: String,
    artifact_identities: Vec<String>,
}

impl Disclosure {
    /// Build the disclosure from a manifest that already passed human local
    /// approval and schema validation.
    ///
    /// # Errors
    ///
    /// Returns a manifest error if the publication state is not bounded.
    pub fn from_manifest(manifest: &ProviderManifest) -> Result<Self, ManagerError> {
        if !matches!(manifest.publication_status(), "blocked" | "approved") {
            return Err(ManagerError::ManifestInvalid);
        }
        let budgets = manifest.resource_budgets();
        Ok(Self {
            manifest_digest: manifest.artifact_set_digest().to_string(),
            transfer_bytes: budgets.transfer_bytes(),
            active_bytes: budgets.active_installed_bytes(),
            lifecycle_bytes: budgets.lifecycle_bytes(),
            publication: manifest.publication_status().to_string(),
            runner_identity: manifest.runner().disclosure_identity(),
            artifact_identities: manifest
                .artifacts()
                .iter()
                .map(crate::manifest::ModelArtifact::disclosure_identity)
                .collect(),
        })
    }

    /// Render only fixed labels, safe sizes, scope and the exact consent digest.
    pub fn render(&self) -> String {
        let artifacts = self.artifact_identities.join(",");
        format!(
            "profile=bergamot-en-es-linux-x86_64-v1\npurpose=offline_english_to_spanish_translation\nlanguage=en-es\nscope=user_xdg_data\nruntime_source=mozilla_translations_pinned\nrunner={}\nartifacts={}\nacquisition_network=exact_https_only\nnormal_translation_network=none\ntransfer_bytes_max={}\nactive_bytes_max={}\nlifecycle_bytes_max={}\npublication={}\nconsent_digest={}\n",
            self.runner_identity,
            artifacts,
            self.transfer_bytes,
            self.active_bytes,
            self.lifecycle_bytes,
            self.publication,
            self.manifest_digest
        )
    }
}
