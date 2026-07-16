use std::fmt;
use std::fs::{self, File};
use std::io::Read;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};

use serde::Deserialize;
use sha2::{Digest as _, Sha256};

use crate::{
    EmbeddedProcessRunner, ErrorCode, Provider, ProviderRequest, ProviderResponse, TranslateFailure,
};

const PROFILE_ID: &str = "bergamot-en-es-linux-x86_64-v1";
const PRODUCT_DIRECTORY: &str = "zed-en-es-translator";
const PROVIDER_DIRECTORY: &str = "embedded";

/// Offline provider backed by one already-verified immutable artifact set.
#[derive(Clone)]
pub struct EmbeddedProcessProvider {
    runner: EmbeddedProcessRunner,
    lease_path: Option<PathBuf>,
}

impl EmbeddedProcessProvider {
    /// Resolve the fixed current set from the product-owned XDG data root.
    ///
    /// This operation is offline and does not prepare, repair, or update state.
    ///
    /// # Errors
    ///
    /// Returns a content-free readiness error when state or artifact identity
    /// is missing, unsafe, incompatible, or corrupt.
    pub fn from_installed() -> Result<Self, TranslateFailure> {
        let root = xdg_root().ok_or_else(not_configured)?;
        Self::from_storage_root(&root)
    }

    /// Construct a provider from an already-validated test process boundary.
    #[doc(hidden)]
    pub const fn from_verified_runner(runner: EmbeddedProcessRunner) -> Self {
        Self {
            runner,
            lease_path: None,
        }
    }

    #[doc(hidden)]
    pub fn from_storage_root(root: &Path) -> Result<Self, TranslateFailure> {
        validate_private_root(root)?;
        let _state_lock = acquire_shared_lock(&root.join("lifecycle.lock"))?;
        let state: InstalledState = read_strict_json(&root.join("state.json"))?;
        state.validate()?;
        let current = state.current.ok_or_else(not_configured)?;
        let set_path = root.join("sets").join(format!("{current}.json"));
        let set: InstalledSet = read_strict_json(&set_path)?;
        set.validate(&current)?;

        let runner_path = validate_object(root, &set.runner)?;
        let mut arguments = Vec::with_capacity(set.artifacts.len() * 2);
        for artifact in &set.artifacts {
            validate_object(root, artifact)?;
            arguments.push(format!("--{}", argument_role(&artifact.role)?));
            arguments.push(object_relative_path(artifact)?);
        }

        let runner = EmbeddedProcessRunner::from_verified_invocation(
            runner_path,
            root.to_path_buf(),
            arguments,
        )?;
        Ok(Self {
            runner,
            lease_path: Some(root.join("lease.lock")),
        })
    }
}

impl Provider for EmbeddedProcessProvider {
    fn translate(&self, request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        let _lease = self
            .lease_path
            .as_deref()
            .map(acquire_inference_lease)
            .transpose()?;
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
struct InstalledState {
    schema_version: u32,
    generation: u64,
    profile_id: String,
    current: Option<String>,
    previous: Option<String>,
    candidate: Option<String>,
    last_operation: String,
    last_outcome: String,
}

impl InstalledState {
    fn validate(&self) -> Result<(), TranslateFailure> {
        let references = [
            self.current.as_deref(),
            self.previous.as_deref(),
            self.candidate.as_deref(),
        ];
        if self.schema_version != 1
            || self.profile_id != PROFILE_ID
            || references
                .into_iter()
                .flatten()
                .any(|value| !is_sha256(value))
            || self.current == self.previous
            || self.current == self.candidate
            || self.last_operation.len() > 32
            || self.last_outcome.len() > 32
        {
            return Err(not_configured());
        }
        let _ = self.generation;
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InstalledSet {
    schema_version: u32,
    manifest_digest: String,
    profile_id: String,
    runner: InstalledObject,
    artifacts: Vec<InstalledObject>,
    verification_state: String,
    offline_smoke: String,
    resource_gate: String,
    license_gate: String,
}

impl InstalledSet {
    fn validate(&self, current: &str) -> Result<(), TranslateFailure> {
        if self.schema_version != 1
            || self.manifest_digest != current
            || self.profile_id != PROFILE_ID
            || self.runner.role != "runner"
            || self.runner.installed_name != "translator-embedded-runtime"
            || self.artifacts.len() != 3
            || self.verification_state != "verified"
            || self.offline_smoke != "passed"
            || self.resource_gate != "passed"
            || self.license_gate != "complete"
        {
            return Err(not_configured());
        }
        let mut roles = self
            .artifacts
            .iter()
            .map(|artifact| artifact.role.as_str())
            .collect::<Vec<_>>();
        roles.sort_unstable();
        if roles != ["lexical_shortlist", "model", "vocabulary"] {
            return Err(not_configured());
        }
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InstalledObject {
    role: String,
    object_digest: String,
    installed_name: String,
    installed_size: u64,
}

fn xdg_root() -> Option<PathBuf> {
    let data_home = std::env::var_os("XDG_DATA_HOME")
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME")
                .filter(|value| !value.is_empty())
                .map(|home| PathBuf::from(home).join(".local/share"))
        })?;
    Some(data_home.join(PRODUCT_DIRECTORY).join(PROVIDER_DIRECTORY))
}

fn validate_private_root(root: &Path) -> Result<(), TranslateFailure> {
    let metadata = fs::symlink_metadata(root).map_err(|_| not_configured())?;
    // SAFETY: `geteuid` has no arguments and no safety preconditions.
    let effective_uid = unsafe { libc::geteuid() };
    if metadata.file_type().is_symlink()
        || !metadata.is_dir()
        || metadata.uid() != effective_uid
        || metadata.permissions().mode() & 0o077 != 0
    {
        return Err(not_configured());
    }
    Ok(())
}

fn read_strict_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, TranslateFailure> {
    let metadata = fs::symlink_metadata(path).map_err(|_| not_configured())?;
    // SAFETY: `geteuid` has no arguments and no safety preconditions.
    let effective_uid = unsafe { libc::geteuid() };
    if metadata.file_type().is_symlink()
        || !metadata.is_file()
        || metadata.uid() != effective_uid
        || metadata.nlink() != 1
        || metadata.permissions().mode() & 0o077 != 0
        || metadata.len() > 64 * 1024
    {
        return Err(not_configured());
    }
    let input = fs::read(path).map_err(|_| not_configured())?;
    serde_json::from_slice(&input).map_err(|_| not_configured())
}

fn validate_object(root: &Path, object: &InstalledObject) -> Result<PathBuf, TranslateFailure> {
    if !is_sha256(&object.object_digest)
        || !is_safe_basename(&object.installed_name)
        || object.installed_size == 0
    {
        return Err(not_configured());
    }
    let path = root
        .join("objects")
        .join(&object.object_digest)
        .join(&object.installed_name);
    let metadata = fs::symlink_metadata(&path).map_err(|_| not_configured())?;
    // SAFETY: `geteuid` has no arguments and no safety preconditions.
    let effective_uid = unsafe { libc::geteuid() };
    if metadata.file_type().is_symlink()
        || !metadata.is_file()
        || metadata.uid() != effective_uid
        || metadata.nlink() != 1
        || metadata.permissions().mode() & 0o022 != 0
        || metadata.len() != object.installed_size
        || sha256_file(&path)? != object.object_digest
    {
        return Err(not_configured());
    }
    Ok(path)
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
    Ok(digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

fn object_relative_path(object: &InstalledObject) -> Result<String, TranslateFailure> {
    if !is_sha256(&object.object_digest) || !is_safe_basename(&object.installed_name) {
        return Err(not_configured());
    }
    Ok(format!(
        "objects/{}/{}",
        object.object_digest, object.installed_name
    ))
}

fn argument_role(role: &str) -> Result<&'static str, TranslateFailure> {
    match role {
        "model" => Ok("model"),
        "vocabulary" => Ok("vocabulary"),
        "lexical_shortlist" => Ok("lexical-shortlist"),
        _ => Err(not_configured()),
    }
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn is_safe_basename(value: &str) -> bool {
    !value.is_empty()
        && value != "."
        && value != ".."
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

fn not_configured() -> TranslateFailure {
    TranslateFailure::new(
        ErrorCode::ProviderNotConfigured,
        "Embedded provider artifacts are not ready.",
    )
}

fn acquire_inference_lease(path: &Path) -> Result<File, TranslateFailure> {
    acquire_shared_lock(path)
}

fn acquire_shared_lock(path: &Path) -> Result<File, TranslateFailure> {
    let metadata = fs::symlink_metadata(path).map_err(|_| not_configured())?;
    // SAFETY: `geteuid` has no arguments and no safety preconditions.
    let effective_uid = unsafe { libc::geteuid() };
    if metadata.file_type().is_symlink()
        || !metadata.is_file()
        || metadata.uid() != effective_uid
        || metadata.nlink() != 1
        || metadata.permissions().mode() & 0o077 != 0
    {
        return Err(not_configured());
    }
    let file = File::open(path).map_err(|_| not_configured())?;
    fs4::FileExt::try_lock_shared(&file).map_err(|_| {
        TranslateFailure::new(
            ErrorCode::ProviderTimeout,
            "Embedded provider installation is busy.",
        )
    })?;
    Ok(file)
}
