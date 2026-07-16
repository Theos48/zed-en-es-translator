use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use sha2::{Digest as _, Sha256};
use translator_core::{EmbeddedProcessRunner, Language, ProviderRequest, Tone};

use crate::error::ManagerError;
use crate::locking::SharedStateLock;
use crate::state::InstallationState;
use crate::storage::{validate_private_directory, validate_private_file, StorageRoot};

/// Safe bounded readiness metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusReport {
    ready: bool,
    generation: u64,
    previous: bool,
    candidate: bool,
    installed_bytes: u64,
}

impl StatusReport {
    /// Render no path, digest, content, or host identity.
    pub fn render(self) -> String {
        format!(
            "provider_status={} profile=bergamot-en-es-linux-x86_64-v1 generation={} previous={} candidate={} installed_bytes={}\n",
            if self.ready { "ready" } else { "absent" },
            self.generation,
            self.previous,
            self.candidate,
            self.installed_bytes
        )
    }
}

/// Read strict installation state and return bounded safe metadata.
///
/// # Errors
///
/// Unsafe storage and invalid state fail closed.
pub fn status(root: &Path) -> Result<StatusReport, ManagerError> {
    if !root.exists() {
        return Ok(StatusReport {
            ready: false,
            generation: 0,
            previous: false,
            candidate: false,
            installed_bytes: 0,
        });
    }
    StorageRoot::validate_existing(root)?;
    let _state_lock = SharedStateLock::try_acquire(root)?;
    let state = read_state(root)?;
    let (current, previous, candidate) = state.references();
    let installed_bytes = current.map_or(Ok(0), |digest| installed_bytes(root, digest))?;
    Ok(StatusReport {
        ready: current.is_some(),
        generation: state.generation(),
        previous: previous.is_some(),
        candidate: candidate.is_some(),
        installed_bytes,
    })
}

/// Rehash the complete current immutable set without network fallback.
///
/// # Errors
///
/// Missing, unsafe, malformed, corrupt, or linked objects return stable errors.
pub fn verify(root: &Path) -> Result<(), ManagerError> {
    StorageRoot::validate_existing(root)?;
    let _state_lock = SharedStateLock::try_acquire(root)?;
    let state = read_state(root)?;
    let current = state.references().0.ok_or(ManagerError::StateInvalid)?;
    verify_digest(root, current)
}

pub(crate) fn verify_digest(root: &Path, digest: &str) -> Result<(), ManagerError> {
    let set: InstalledSet =
        read_json(&root.join("sets").join(format!("{digest}.json")), 64 * 1024)?;
    if set.schema_version != 1
        || set.manifest_digest != digest
        || set.profile_id != "bergamot-en-es-linux-x86_64-v1"
        || set.verification_state != "verified"
        || set.offline_smoke != "passed"
        || set.resource_gate != "passed"
        || set.license_gate != "complete"
        || set.runner.role != "runner"
        || set.artifacts.len() != 3
    {
        return Err(ManagerError::StateInvalid);
    }
    verify_object(root, &set.runner)?;
    for object in &set.artifacts {
        verify_object(root, object)?;
    }
    offline_smoke(root, &set)?;
    Ok(())
}

fn offline_smoke(root: &Path, set: &InstalledSet) -> Result<(), ManagerError> {
    let executable = object_path(root, &set.runner);
    let mut arguments = Vec::with_capacity(6);
    for (role, flag) in [
        ("model", "--model"),
        ("vocabulary", "--vocabulary"),
        ("lexical_shortlist", "--lexical-shortlist"),
    ] {
        let object = set
            .artifacts
            .iter()
            .find(|object| object.role == role)
            .ok_or(ManagerError::StateInvalid)?;
        arguments.push(flag.to_string());
        arguments.push(format!(
            "objects/{}/{}",
            object.object_digest, object.installed_name
        ));
    }
    let process =
        EmbeddedProcessRunner::from_verified_invocation(executable, root.to_path_buf(), arguments)
            .map_err(|_| ManagerError::IntegrityFailed)?;
    let source = "This public synthetic check verifies offline translation.";
    let request = ProviderRequest::new(
        vec![source.to_string()],
        Language::English,
        Language::Spanish,
        Tone::TechnicalNeutral,
    )
    .map_err(|_| ManagerError::IntegrityFailed)?;
    let response = process
        .run(&request)
        .map_err(|_| ManagerError::IntegrityFailed)?;
    if response.translated_segments.len() != 1
        || response.translated_segments[0].trim().is_empty()
        || response.translated_segments[0] == source
    {
        return Err(ManagerError::IntegrityFailed);
    }
    Ok(())
}

fn read_state(root: &Path) -> Result<InstallationState, ManagerError> {
    let path = root.join("state.json");
    validate_private_file(&path)?;
    let input = fs::read_to_string(path).map_err(|_| ManagerError::StateInvalid)?;
    InstallationState::from_json(&input)
}

fn installed_bytes(root: &Path, digest: &str) -> Result<u64, ManagerError> {
    let set: InstalledSet =
        read_json(&root.join("sets").join(format!("{digest}.json")), 64 * 1024)?;
    set.artifacts
        .iter()
        .try_fold(set.runner.installed_size, |total, object| {
            total
                .checked_add(object.installed_size)
                .ok_or(ManagerError::StateInvalid)
        })
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

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InstalledObject {
    role: String,
    object_digest: String,
    installed_name: String,
    installed_size: u64,
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path, limit: u64) -> Result<T, ManagerError> {
    validate_private_directory(path.parent().ok_or(ManagerError::StateInvalid)?)?;
    let metadata = validate_private_file(path)?;
    if metadata.len() > limit {
        return Err(ManagerError::StateInvalid);
    }
    let input = fs::read(path).map_err(|_| ManagerError::StateInvalid)?;
    serde_json::from_slice(&input).map_err(|_| ManagerError::StateInvalid)
}

fn verify_object(root: &Path, object: &InstalledObject) -> Result<(), ManagerError> {
    if !is_sha256(&object.object_digest) || !is_safe_basename(&object.installed_name) {
        return Err(ManagerError::StateInvalid);
    }
    let path = object_path(root, object);
    validate_private_directory(&root.join("objects"))?;
    validate_private_directory(&root.join("objects").join(&object.object_digest))?;
    let metadata = validate_private_file(&path)?;
    if metadata.len() != object.installed_size || sha256_file(&path)? != object.object_digest {
        return Err(ManagerError::IntegrityFailed);
    }
    Ok(())
}

fn object_path(root: &Path, object: &InstalledObject) -> PathBuf {
    root.join("objects")
        .join(&object.object_digest)
        .join(&object.installed_name)
}

fn sha256_file(path: &Path) -> Result<String, ManagerError> {
    let mut file = File::open(path).map_err(|_| ManagerError::IntegrityFailed)?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 32 * 1024];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|_| ManagerError::IntegrityFailed)?;
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

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn is_safe_basename(value: &str) -> bool {
    !value.is_empty()
        && value != "."
        && value != ".."
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}
