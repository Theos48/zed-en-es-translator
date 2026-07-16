use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::Read;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Component, Path};

use serde::Deserialize;
use sha2::{Digest as _, Sha256};

use crate::{
    EmbeddedProcessRunner, ErrorCode, Provider, ProviderRequest, ProviderResponse, TranslateFailure,
};

const MAXIMUM_MANIFEST_BYTES: u64 = 64 * 1024;
const MAXIMUM_PACKAGE_BYTES: u64 = 128 * 1024 * 1024;
const PLATFORM: &str = "linux-x86_64";

/// Offline provider backed by the immutable package adjacent to `translator-lsp`.
#[derive(Clone)]
pub struct EmbeddedProcessProvider {
    runner: EmbeddedProcessRunner,
}

impl EmbeddedProcessProvider {
    /// Resolve and verify the package containing the currently running server.
    ///
    /// # Errors
    ///
    /// Returns a content-free readiness error if the executable is not the
    /// expected server inside a complete verified package.
    pub fn from_current_executable() -> Result<Self, TranslateFailure> {
        let executable = std::env::current_exe().map_err(|_| not_configured())?;
        let bin = executable.parent().ok_or_else(not_configured)?;
        if executable.file_name().and_then(|value| value.to_str()) != Some("translator-lsp")
            || bin.file_name().and_then(|value| value.to_str()) != Some("bin")
        {
            return Err(not_configured());
        }
        let package_root = bin.parent().ok_or_else(not_configured)?;
        Self::from_package_root(package_root)
    }

    /// Verify one package root and construct its fixed native invocation.
    #[doc(hidden)]
    pub fn from_package_root(package_root: &Path) -> Result<Self, TranslateFailure> {
        validate_directory(package_root)?;
        let manifest: InstalledPackage = read_manifest(&package_root.join("installed.json"))?;
        manifest.validate(package_root)?;

        let artifacts = manifest
            .artifacts
            .iter()
            .map(|artifact| (artifact.role.as_str(), artifact))
            .collect::<BTreeMap<_, _>>();
        let runner_path = package_root.join(&artifacts["native_runner"].path);
        let arguments = vec![
            "--model".to_string(),
            artifacts["model"].path.clone(),
            "--vocabulary".to_string(),
            artifacts["vocabulary"].path.clone(),
            "--lexical-shortlist".to_string(),
            artifacts["lexical_shortlist"].path.clone(),
        ];
        let runner = EmbeddedProcessRunner::from_verified_invocation(
            runner_path,
            package_root.to_path_buf(),
            arguments,
        )?;
        Ok(Self { runner })
    }

    /// Construct a provider from an already-validated test process boundary.
    #[doc(hidden)]
    pub const fn from_verified_runner(runner: EmbeddedProcessRunner) -> Self {
        Self { runner }
    }
}

impl Provider for EmbeddedProcessProvider {
    fn translate(&self, request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        self.runner.run(request)
    }
}

impl fmt::Debug for EmbeddedProcessProvider {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EmbeddedProcessProvider")
            .finish_non_exhaustive()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InstalledPackage {
    schema_version: u32,
    package_id: String,
    package_version: String,
    platform: String,
    source_language: String,
    target_language: String,
    wire_version: u32,
    state: String,
    artifacts: Vec<InstalledArtifact>,
}

impl InstalledPackage {
    fn validate(&self, package_root: &Path) -> Result<(), TranslateFailure> {
        let package_directory = package_root
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(not_configured)?;
        if self.schema_version != 1
            || self.package_id != package_directory
            || !is_safe_id(&self.package_id)
            || !is_semver(&self.package_version)
            || self.platform != PLATFORM
            || self.source_language != "en"
            || self.target_language != "es"
            || self.wire_version != 1
            || self.state != "verified"
            || self.artifacts.len() != 5
        {
            return Err(not_configured());
        }

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
        let mut actual = BTreeMap::new();
        let mut total = 0_u64;
        for artifact in &self.artifacts {
            if actual.insert(artifact.role.as_str(), artifact).is_some() {
                return Err(not_configured());
            }
            let Some((path, executable)) = expected.get(artifact.role.as_str()) else {
                return Err(not_configured());
            };
            if artifact.path != *path || artifact.executable != *executable {
                return Err(not_configured());
            }
            total = total
                .checked_add(artifact.installed_size)
                .ok_or_else(not_configured)?;
            validate_artifact(package_root, artifact)?;
        }
        if actual.len() != expected.len() || total > MAXIMUM_PACKAGE_BYTES {
            return Err(not_configured());
        }
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InstalledArtifact {
    role: String,
    path: String,
    installed_size: u64,
    installed_sha256: String,
    executable: bool,
}

fn read_manifest(path: &Path) -> Result<InstalledPackage, TranslateFailure> {
    let metadata = fs::symlink_metadata(path).map_err(|_| not_configured())?;
    if metadata.file_type().is_symlink()
        || !metadata.is_file()
        || metadata.nlink() != 1
        || metadata.len() == 0
        || metadata.len() > MAXIMUM_MANIFEST_BYTES
        || metadata.permissions().mode() & 0o022 != 0
    {
        return Err(not_configured());
    }
    let input = fs::read(path).map_err(|_| not_configured())?;
    serde_json::from_slice(&input).map_err(|_| not_configured())
}

fn validate_directory(path: &Path) -> Result<(), TranslateFailure> {
    let metadata = fs::symlink_metadata(path).map_err(|_| not_configured())?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(not_configured());
    }
    Ok(())
}

fn validate_artifact(
    package_root: &Path,
    artifact: &InstalledArtifact,
) -> Result<(), TranslateFailure> {
    if !is_safe_relative_path(&artifact.path)
        || artifact.installed_size == 0
        || !is_sha256(&artifact.installed_sha256)
    {
        return Err(not_configured());
    }
    let path = package_root.join(&artifact.path);
    let parent = path.parent().ok_or_else(not_configured)?;
    validate_directory(parent)?;
    let metadata = fs::symlink_metadata(&path).map_err(|_| not_configured())?;
    let executable_bits = metadata.permissions().mode() & 0o111;
    if metadata.file_type().is_symlink()
        || !metadata.is_file()
        || metadata.nlink() != 1
        || metadata.len() != artifact.installed_size
        || metadata.permissions().mode() & 0o022 != 0
        || (artifact.executable && executable_bits == 0)
        || (!artifact.executable && executable_bits != 0)
        || sha256_file(&path)? != artifact.installed_sha256
    {
        return Err(not_configured());
    }
    Ok(())
}

fn sha256_file(path: &Path) -> Result<String, TranslateFailure> {
    let mut file = File::open(path).map_err(|_| not_configured())?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 32 * 1024];
    loop {
        let count = file.read(&mut buffer).map_err(|_| not_configured())?;
        if count == 0 {
            break;
        }
        digest.update(&buffer[..count]);
    }
    let mut output = String::with_capacity(64);
    for byte in digest.finalize() {
        write!(&mut output, "{byte:02x}").map_err(|_| not_configured())?;
    }
    Ok(output)
}

fn is_safe_relative_path(value: &str) -> bool {
    let path = Path::new(value);
    !value.is_empty()
        && !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(name) if !name.is_empty()))
}

fn is_safe_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"._-".contains(&byte)
        })
}

fn is_semver(value: &str) -> bool {
    let components = value.split('.').collect::<Vec<_>>();
    components.len() == 3
        && components.iter().all(|component| {
            !component.is_empty() && component.bytes().all(|byte| byte.is_ascii_digit())
        })
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn not_configured() -> TranslateFailure {
    TranslateFailure::new(
        ErrorCode::ProviderNotConfigured,
        "Embedded provider package is not ready.",
    )
}
