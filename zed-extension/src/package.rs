use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Component, Path};

use serde::{Deserialize, Serialize};

const MAXIMUM_TRANSFER_BYTES: u64 = 64 * 1024 * 1024;
const MAXIMUM_ACTIVE_BYTES: u64 = 128 * 1024 * 1024;
const MAXIMUM_LIFECYCLE_BYTES: u64 = 384 * 1024 * 1024;
const REQUIRED_FREE_BYTES: u64 = 512 * 1024 * 1024;
const MAXIMUM_RSS_BYTES: u64 = 1024 * 1024 * 1024;
const INFERENCE_THREADS: u32 = 4;
const PROVIDER_DEADLINE_MS: u64 = 15_000;

/// Content-free published-package validation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackageError;

impl fmt::Display for PackageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("The translation package lock is invalid.")
    }
}

impl std::error::Error for PackageError {}

/// Strict lock compiled into the marketplace extension.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublishedPackage {
    schema_version: u32,
    package_id: String,
    package_version: String,
    platform: String,
    source_language: String,
    target_language: String,
    server_archive: ServerArchive,
    model_resources: Vec<ModelResource>,
    budgets: PackageBudgets,
    license_bundle: LicenseBundle,
}

impl PublishedPackage {
    /// Parse and semantically validate a complete package lock.
    pub fn parse(input: &str) -> Result<Self, PackageError> {
        let package: Self = serde_json::from_str(input).map_err(|_| PackageError)?;
        package.validate()?;
        Ok(package)
    }

    /// Immutable package generation identifier.
    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    /// Exact files extracted from the server archive.
    pub fn server_files(&self) -> &[ServerFile] {
        &self.server_archive.files
    }

    /// Exact compressed and installed model resources.
    pub fn model_resources(&self) -> &[ModelResource] {
        &self.model_resources
    }

    pub(crate) fn server_archive_url(&self) -> &str {
        &self.server_archive.url
    }

    pub(crate) fn installed_manifest(&self) -> InstalledPackage {
        let mut artifacts = self
            .server_archive
            .files
            .iter()
            .filter(|file| {
                matches!(
                    file.role,
                    ServerRole::LanguageServer | ServerRole::NativeRunner
                )
            })
            .map(|file| InstalledArtifact {
                role: file.role.as_runtime_role().to_string(),
                path: file.path.clone(),
                installed_size: file.installed_size,
                installed_sha256: file.installed_sha256.clone(),
                executable: file.executable,
            })
            .collect::<Vec<_>>();
        artifacts.extend(
            self.model_resources
                .iter()
                .map(|resource| InstalledArtifact {
                    role: resource.role.as_str().to_string(),
                    path: format!("models/{}", resource.installed_name),
                    installed_size: resource.installed_size,
                    installed_sha256: resource.installed_sha256.clone(),
                    executable: false,
                }),
        );
        InstalledPackage {
            schema_version: 1,
            package_id: self.package_id.clone(),
            package_version: self.package_version.clone(),
            platform: self.platform.clone(),
            source_language: self.source_language.clone(),
            target_language: self.target_language.clone(),
            wire_version: 1,
            state: "verified".to_string(),
            artifacts,
        }
    }

    fn validate(&self) -> Result<(), PackageError> {
        if self.schema_version != 1
            || !is_safe_id(&self.package_id)
            || !is_semver(&self.package_version)
            || self.platform != "linux-x86_64"
            || self.source_language != "en"
            || self.target_language != "es"
            || self.server_archive.archive_type != "gzip_tar"
            || !is_https_url(&self.server_archive.url)
            || !self
                .server_archive
                .url
                .starts_with("https://github.com/Theos48/zed-en-es-translator/releases/download/")
            || self.server_archive.files.len() < 5
            || self.server_archive.files.len() > 32
            || self.model_resources.len() != 3
            || !self.budgets.is_fixed()
            || self.license_bundle.extension_spdx != "MIT"
            || self.license_bundle.required_paths.len() < 3
        {
            return Err(PackageError);
        }

        let mut server_roles = BTreeMap::<&str, usize>::new();
        let mut server_paths = BTreeSet::new();
        let mut installed_bytes = 0_u64;
        for file in &self.server_archive.files {
            file.validate()?;
            *server_roles.entry(file.role.as_str()).or_default() += 1;
            if !server_paths.insert(file.path.as_str()) {
                return Err(PackageError);
            }
            installed_bytes = installed_bytes
                .checked_add(file.installed_size)
                .ok_or(PackageError)?;
        }
        if server_roles.get("language_server") != Some(&1)
            || server_roles.get("native_runner") != Some(&1)
            || server_roles.get("notice").copied().unwrap_or_default() < 1
            || server_roles
                .get("source_instructions")
                .copied()
                .unwrap_or_default()
                < 1
            || server_roles.get("license").copied().unwrap_or_default() < 1
        {
            return Err(PackageError);
        }

        let mut model_roles = BTreeSet::new();
        let mut transfer_bytes = installed_bytes;
        for resource in &self.model_resources {
            resource.validate()?;
            if !model_roles.insert(resource.role.as_str()) {
                return Err(PackageError);
            }
            transfer_bytes = transfer_bytes
                .checked_add(resource.compressed_size)
                .ok_or(PackageError)?;
            installed_bytes = installed_bytes
                .checked_add(resource.installed_size)
                .ok_or(PackageError)?;
        }
        if model_roles != BTreeSet::from(["lexical_shortlist", "model", "vocabulary"])
            || transfer_bytes > MAXIMUM_TRANSFER_BYTES
            || installed_bytes > MAXIMUM_ACTIVE_BYTES
        {
            return Err(PackageError);
        }

        let required = self
            .license_bundle
            .required_paths
            .iter()
            .collect::<BTreeSet<_>>();
        if required.len() != self.license_bundle.required_paths.len()
            || required.iter().any(|path| {
                !is_safe_relative_path(path)
                    || !self.server_archive.files.iter().any(|file| {
                        &file.path == *path
                            && matches!(
                                file.role,
                                ServerRole::Notice
                                    | ServerRole::License
                                    | ServerRole::SourceInstructions
                            )
                    })
            })
        {
            return Err(PackageError);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ServerArchive {
    url: String,
    archive_type: String,
    files: Vec<ServerFile>,
}

/// One exact file extracted from the project release archive.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerFile {
    role: ServerRole,
    path: String,
    installed_size: u64,
    installed_sha256: String,
    executable: bool,
    spdx_conclusion: String,
    source_url: String,
}

impl ServerFile {
    pub(crate) fn path(&self) -> &str {
        &self.path
    }

    pub(crate) const fn size(&self) -> u64 {
        self.installed_size
    }

    pub(crate) fn sha256(&self) -> &str {
        &self.installed_sha256
    }

    pub(crate) const fn executable(&self) -> bool {
        self.executable
    }

    fn validate(&self) -> Result<(), PackageError> {
        let role_executable = matches!(
            self.role,
            ServerRole::LanguageServer | ServerRole::NativeRunner
        );
        if !is_safe_relative_path(&self.path)
            || self.installed_size == 0
            || self.installed_size > MAXIMUM_ACTIVE_BYTES
            || !is_sha256(&self.installed_sha256)
            || self.executable != role_executable
            || self.spdx_conclusion.len() < 3
            || self.spdx_conclusion.len() > 256
            || !is_https_url(&self.source_url)
        {
            return Err(PackageError);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ServerRole {
    LanguageServer,
    NativeRunner,
    Notice,
    SourceInstructions,
    License,
}

impl ServerRole {
    const fn as_str(self) -> &'static str {
        match self {
            Self::LanguageServer => "language_server",
            Self::NativeRunner => "native_runner",
            Self::Notice => "notice",
            Self::SourceInstructions => "source_instructions",
            Self::License => "license",
        }
    }

    const fn as_runtime_role(self) -> &'static str {
        self.as_str()
    }
}

/// One exact Mozilla model download and its installed identity.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelResource {
    role: ModelRole,
    record_id: String,
    #[serde(default)]
    record_version: Option<String>,
    #[serde(default)]
    architecture: Option<String>,
    url: String,
    compressed_name: String,
    compressed_size: u64,
    compressed_sha256: String,
    installed_name: String,
    installed_size: u64,
    installed_sha256: String,
    spdx_conclusion: String,
    license_url: String,
}

impl ModelResource {
    pub(crate) fn url(&self) -> &str {
        &self.url
    }

    pub(crate) fn compressed_name(&self) -> &str {
        &self.compressed_name
    }

    pub(crate) const fn compressed_size(&self) -> u64 {
        self.compressed_size
    }

    pub(crate) fn compressed_sha256(&self) -> &str {
        &self.compressed_sha256
    }

    pub(crate) fn installed_name(&self) -> &str {
        &self.installed_name
    }

    pub(crate) const fn installed_size(&self) -> u64 {
        self.installed_size
    }

    pub(crate) fn installed_sha256(&self) -> &str {
        &self.installed_sha256
    }

    fn validate(&self) -> Result<(), PackageError> {
        if !is_uuid(&self.record_id)
            || self
                .record_version
                .as_ref()
                .is_some_and(|value| value.is_empty() || value.len() > 32)
            || self
                .architecture
                .as_ref()
                .is_some_and(|value| value.is_empty() || value.len() > 64)
            || !is_https_url(&self.url)
            || !is_safe_basename(&self.compressed_name)
            || self.compressed_size == 0
            || self.compressed_size > MAXIMUM_TRANSFER_BYTES
            || !is_sha256(&self.compressed_sha256)
            || !is_safe_basename(&self.installed_name)
            || self.installed_size == 0
            || self.installed_size > MAXIMUM_ACTIVE_BYTES
            || !is_sha256(&self.installed_sha256)
            || self.spdx_conclusion != "MPL-2.0"
            || !is_https_url(&self.license_url)
        {
            return Err(PackageError);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ModelRole {
    Model,
    Vocabulary,
    LexicalShortlist,
}

impl ModelRole {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Model => "model",
            Self::Vocabulary => "vocabulary",
            Self::LexicalShortlist => "lexical_shortlist",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct PackageBudgets {
    maximum_transfer_bytes: u64,
    maximum_active_installed_bytes: u64,
    maximum_lifecycle_bytes: u64,
    required_free_bytes: u64,
    peak_rss_bytes: u64,
    inference_threads: u32,
    provider_deadline_ms: u64,
}

impl PackageBudgets {
    const fn is_fixed(&self) -> bool {
        self.maximum_transfer_bytes == MAXIMUM_TRANSFER_BYTES
            && self.maximum_active_installed_bytes == MAXIMUM_ACTIVE_BYTES
            && self.maximum_lifecycle_bytes == MAXIMUM_LIFECYCLE_BYTES
            && self.required_free_bytes == REQUIRED_FREE_BYTES
            && self.peak_rss_bytes == MAXIMUM_RSS_BYTES
            && self.inference_threads == INFERENCE_THREADS
            && self.provider_deadline_ms == PROVIDER_DEADLINE_MS
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct LicenseBundle {
    extension_spdx: String,
    required_paths: Vec<String>,
}

/// Strict manifest written only after all installed files verify.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct InstalledPackage {
    pub(crate) schema_version: u32,
    pub(crate) package_id: String,
    pub(crate) package_version: String,
    pub(crate) platform: String,
    pub(crate) source_language: String,
    pub(crate) target_language: String,
    pub(crate) wire_version: u32,
    pub(crate) state: String,
    pub(crate) artifacts: Vec<InstalledArtifact>,
}

impl InstalledPackage {
    pub(crate) fn parse(input: &[u8]) -> Result<Self, PackageError> {
        let manifest: Self = serde_json::from_slice(input).map_err(|_| PackageError)?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub(crate) fn to_json(&self) -> Result<Vec<u8>, PackageError> {
        serde_json::to_vec_pretty(self).map_err(|_| PackageError)
    }

    fn validate(&self) -> Result<(), PackageError> {
        let expected = BTreeMap::from([
            ("language_server", ("bin/translator-lsp", true)),
            ("native_runner", ("bin/translator-embedded-runtime", true)),
            ("model", ("models/model.enes.intgemm.alphas.bin", false)),
            ("vocabulary", ("models/vocab.enes.spm", false)),
            (
                "lexical_shortlist",
                ("models/lex.50.50.enes.s2t.bin", false),
            ),
        ]);
        if self.schema_version != 1
            || !is_safe_id(&self.package_id)
            || !is_semver(&self.package_version)
            || self.platform != "linux-x86_64"
            || self.source_language != "en"
            || self.target_language != "es"
            || self.wire_version != 1
            || self.state != "verified"
            || self.artifacts.len() != expected.len()
        {
            return Err(PackageError);
        }
        let mut roles = BTreeSet::new();
        for artifact in &self.artifacts {
            let Some((path, executable)) = expected.get(artifact.role.as_str()) else {
                return Err(PackageError);
            };
            if !roles.insert(artifact.role.as_str())
                || artifact.path != *path
                || artifact.executable != *executable
                || artifact.installed_size == 0
                || artifact.installed_size > MAXIMUM_ACTIVE_BYTES
                || !is_sha256(&artifact.installed_sha256)
            {
                return Err(PackageError);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct InstalledArtifact {
    pub(crate) role: String,
    pub(crate) path: String,
    pub(crate) installed_size: u64,
    pub(crate) installed_sha256: String,
    pub(crate) executable: bool,
}

fn is_safe_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"._-".contains(&byte)
        })
}

fn is_semver(value: &str) -> bool {
    let mut components = value.split('.');
    let valid = components.by_ref().take(3).all(|component| {
        !component.is_empty() && component.bytes().all(|byte| byte.is_ascii_digit())
    });
    valid && components.next().is_none() && value.matches('.').count() == 2
}

fn is_https_url(value: &str) -> bool {
    value.starts_with("https://")
        && value.len() > "https://".len()
        && !value.contains(['?', '#'])
        && !value.chars().any(char::is_whitespace)
}

fn is_safe_relative_path(value: &str) -> bool {
    let path = Path::new(value);
    !value.is_empty()
        && !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(name) if !name.is_empty()))
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b'/'))
}

fn is_safe_basename(value: &str) -> bool {
    is_safe_relative_path(value) && !value.contains('/') && value != "." && value != ".."
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value != "0".repeat(64)
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn is_uuid(value: &str) -> bool {
    value.len() == 36
        && value.bytes().enumerate().all(|(index, byte)| {
            if matches!(index, 8 | 13 | 18 | 23) {
                byte == b'-'
            } else {
                byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)
            }
        })
}
