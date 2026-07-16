use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Cursor, ErrorKind, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};

use serde_json::json;
use sha2::{Digest as _, Sha256};
use translator_core::{EmbeddedProcessRunner, Language, ProviderRequest, Tone};

use crate::acquisition::AcquisitionPolicy;
use crate::artifact::expand_zstandard;
use crate::error::ManagerError;
use crate::locking::{ensure_lock_files, ExclusiveLifecycleLock, SharedStateLock};
use crate::manifest::{ModelArtifact, ProviderManifest};
use crate::state::InstallationState;
use crate::status::verify_digest;
use crate::storage::{validate_private_directory, validate_private_file, StorageRoot};

/// One already-acquired compressed fixture bound to its manifest role.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledArtifact {
    pub role: String,
    pub compressed: Vec<u8>,
}

/// Atomic embedded-provider lifecycle rooted in one fixed XDG directory.
#[derive(Debug, Clone)]
pub struct Lifecycle {
    root: PathBuf,
}

/// Durable lifecycle boundaries exposed only for controlled interruption tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleCheckpoint {
    /// The private operation staging directory exists.
    StagingCreated,
    /// Every verified object has been materialized below staging.
    ObjectsStaged,
    /// Staged immutable objects have been finalized in the object store.
    ObjectsFinalized,
    /// The immutable artifact-set record has been durably finalized.
    SetFinalized,
    /// The candidate reference has been durably recorded but not promoted.
    CandidatePersisted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OperationKind {
    Prepare,
    Update,
}

struct LifecycleRequest<'a> {
    operation: OperationKind,
    manifest: &'a ProviderManifest,
    consent: &'a str,
    runner: &'a [u8],
    sources: &'a [ControlledArtifact],
}

struct Finalization<'a> {
    manifest: &'a ProviderManifest,
    runner: &'a [u8],
    installed: &'a [(String, String, Vec<u8>)],
    staging: &'a Path,
    state: InstallationState,
    previous_state: Option<Vec<u8>>,
    idempotent: bool,
}

impl OperationKind {
    const fn staging_prefix(self) -> &'static str {
        match self {
            Self::Prepare => "prepare",
            Self::Update => "update",
        }
    }
}

impl Lifecycle {
    /// Construct lifecycle orchestration for an internally derived root.
    pub const fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// Confirm that an update has a different existing current set before any
    /// runner read or network acquisition begins.
    ///
    /// The mutating update repeats this check under its exclusive lock, so a
    /// concurrent lifecycle change still fails closed.
    ///
    /// # Errors
    ///
    /// Returns a stable storage, state, or busy failure when update cannot
    /// safely proceed.
    pub fn validate_update_preconditions(&self, digest: &str) -> Result<(), ManagerError> {
        if !self.root.exists() {
            return Err(ManagerError::StateInvalid);
        }
        StorageRoot::validate_existing(&self.root)?;
        let _state_lock = SharedStateLock::try_acquire(&self.root)?;
        let state = read_private_state(&self.root.join("state.json"))?;
        match state.references().0 {
            Some(current) if current != digest => Ok(()),
            _ => Err(ManagerError::StateInvalid),
        }
    }

    /// Validate controlled acquired inputs, finalize immutable objects, and
    /// atomically promote a first installation.
    ///
    /// Consent is checked before any storage mutation. This test seam does not
    /// perform network access; production acquisition uses `AcquisitionPolicy`.
    ///
    /// # Errors
    ///
    /// Returns stable consent, integrity, state, or storage failures. Existing
    /// current state is unchanged on every pre-promotion error.
    pub fn prepare(
        &self,
        manifest: &ProviderManifest,
        consent: &str,
        runner: &[u8],
        sources: &[ControlledArtifact],
    ) -> Result<(), ManagerError> {
        self.run_with_checks(
            LifecycleRequest {
                operation: OperationKind::Prepare,
                manifest,
                consent,
                runner,
                sources,
            },
            |_, _| Ok(()),
            verify_digest,
            |_, _| Ok(()),
        )
    }

    /// Validate controlled acquired inputs and atomically replace an existing
    /// installation while retaining the former set for rollback.
    ///
    /// # Errors
    ///
    /// Returns [`ManagerError::StateInvalid`] when there is no existing
    /// current set or the requested set is already current. Other failures use
    /// the same stable classes as [`Self::prepare`].
    pub fn update(
        &self,
        manifest: &ProviderManifest,
        consent: &str,
        runner: &[u8],
        sources: &[ControlledArtifact],
    ) -> Result<(), ManagerError> {
        self.run_with_checks(
            LifecycleRequest {
                operation: OperationKind::Update,
                manifest,
                consent,
                runner,
                sources,
            },
            |_, _| Ok(()),
            verify_digest,
            |_, _| Ok(()),
        )
    }

    /// Prepare production artifacts and require one real offline native smoke
    /// before the atomic state promotion.
    ///
    /// # Errors
    ///
    /// In addition to normal preparation failures, a runner/protocol/quality
    /// smoke failure leaves the former current state unchanged.
    pub fn prepare_with_offline_smoke(
        &self,
        manifest: &ProviderManifest,
        consent: &str,
        runner: &[u8],
        sources: &[ControlledArtifact],
    ) -> Result<(), ManagerError> {
        self.run_with_checks(
            LifecycleRequest {
                operation: OperationKind::Prepare,
                manifest,
                consent,
                runner,
                sources,
            },
            production_offline_smoke,
            verify_digest,
            |_, _| Ok(()),
        )
    }

    /// Update production artifacts and require one real offline native smoke
    /// before atomic candidate promotion.
    ///
    /// # Errors
    ///
    /// In addition to normal update failures, a runner/protocol/quality smoke
    /// failure leaves the former current state unchanged.
    pub fn update_with_offline_smoke(
        &self,
        manifest: &ProviderManifest,
        consent: &str,
        runner: &[u8],
        sources: &[ControlledArtifact],
    ) -> Result<(), ManagerError> {
        self.run_with_checks(
            LifecycleRequest {
                operation: OperationKind::Update,
                manifest,
                consent,
                runner,
                sources,
            },
            production_offline_smoke,
            verify_digest,
            |_, _| Ok(()),
        )
    }

    /// Controlled seam that proves a failed post-promotion verification
    /// restores the former atomic state. Production always supplies the
    /// complete offline verifier.
    #[doc(hidden)]
    pub fn prepare_with_post_promotion_check<F>(
        &self,
        manifest: &ProviderManifest,
        consent: &str,
        runner: &[u8],
        sources: &[ControlledArtifact],
        post_promotion_check: F,
    ) -> Result<(), ManagerError>
    where
        F: FnOnce(&Path, &str) -> Result<(), ManagerError>,
    {
        self.run_with_checks(
            LifecycleRequest {
                operation: OperationKind::Prepare,
                manifest,
                consent,
                runner,
                sources,
            },
            |_, _| Ok(()),
            post_promotion_check,
            |_, _| Ok(()),
        )
    }

    /// Controlled update seam that proves a failed post-promotion check
    /// restores the former atomic state.
    #[doc(hidden)]
    pub fn update_with_post_promotion_check<F>(
        &self,
        manifest: &ProviderManifest,
        consent: &str,
        runner: &[u8],
        sources: &[ControlledArtifact],
        post_promotion_check: F,
    ) -> Result<(), ManagerError>
    where
        F: FnOnce(&Path, &str) -> Result<(), ManagerError>,
    {
        self.run_with_checks(
            LifecycleRequest {
                operation: OperationKind::Update,
                manifest,
                consent,
                runner,
                sources,
            },
            |_, _| Ok(()),
            post_promotion_check,
            |_, _| Ok(()),
        )
    }

    /// Inject a failure at a durable update boundary without adding a
    /// production environment or filesystem seam.
    #[doc(hidden)]
    pub fn update_with_checkpoint<F>(
        &self,
        manifest: &ProviderManifest,
        consent: &str,
        runner: &[u8],
        sources: &[ControlledArtifact],
        checkpoint: F,
    ) -> Result<(), ManagerError>
    where
        F: FnMut(LifecycleCheckpoint, &Path) -> Result<(), ManagerError>,
    {
        self.run_with_checks(
            LifecycleRequest {
                operation: OperationKind::Update,
                manifest,
                consent,
                runner,
                sources,
            },
            |_, _| Ok(()),
            verify_digest,
            checkpoint,
        )
    }

    /// Inject a failure at a durable first-preparation boundary without adding
    /// a production environment or filesystem seam.
    #[doc(hidden)]
    pub fn prepare_with_checkpoint<F>(
        &self,
        manifest: &ProviderManifest,
        consent: &str,
        runner: &[u8],
        sources: &[ControlledArtifact],
        checkpoint: F,
    ) -> Result<(), ManagerError>
    where
        F: FnMut(LifecycleCheckpoint, &Path) -> Result<(), ManagerError>,
    {
        self.run_with_checks(
            LifecycleRequest {
                operation: OperationKind::Prepare,
                manifest,
                consent,
                runner,
                sources,
            },
            |_, _| Ok(()),
            verify_digest,
            checkpoint,
        )
    }

    fn run_with_checks<S, P, C>(
        &self,
        request: LifecycleRequest<'_>,
        offline_smoke: S,
        post_promotion_check: P,
        mut checkpoint: C,
    ) -> Result<(), ManagerError>
    where
        S: FnOnce(&Path, &ProviderManifest) -> Result<(), ManagerError>,
        P: FnOnce(&Path, &str) -> Result<(), ManagerError>,
        C: FnMut(LifecycleCheckpoint, &Path) -> Result<(), ManagerError>,
    {
        let LifecycleRequest {
            operation,
            manifest,
            consent,
            runner,
            sources,
        } = request;
        if consent != manifest.artifact_set_digest() {
            return Err(ManagerError::ConsentRequired);
        }
        validate_runner(manifest, runner)?;
        let installed = validate_artifacts(manifest.artifacts(), sources)?;
        ensure_free_space(
            &self.root,
            manifest.resource_budgets().required_free_bytes(),
        )?;
        if operation == OperationKind::Update && !self.root.exists() {
            return Err(ManagerError::StateInvalid);
        }

        self.create_root()?;
        let _lifecycle_lock = ExclusiveLifecycleLock::try_acquire(&self.root)?;
        let state_path = self.root.join("state.json");
        let (mut state, mut previous_state) = match fs::symlink_metadata(&state_path) {
            Ok(_) => {
                let content = read_private_file(&state_path)?;
                (parse_state(&content)?, Some(content))
            }
            Err(error) if error.kind() == ErrorKind::NotFound => {
                (InstallationState::empty(manifest.profile_id()), None)
            }
            Err(_) => return Err(ManagerError::StorageFailed),
        };
        if state.references().2.is_some() {
            state.reject_candidate()?;
            let recovered = state.to_json()?;
            atomic_write(&state_path, &recovered, 0o600)?;
            previous_state = Some(recovered);
        }
        let idempotent = match (operation, state.references().0) {
            (OperationKind::Prepare, None) => false,
            (OperationKind::Prepare, Some(current))
                if current == manifest.artifact_set_digest() =>
            {
                true
            }
            (OperationKind::Update, Some(current)) if current != manifest.artifact_set_digest() => {
                false
            }
            _ => return Err(ManagerError::StateInvalid),
        };
        let staging = self.root.join("staging").join(format!(
            "{}-{}",
            operation.staging_prefix(),
            std::process::id()
        ));
        match fs::symlink_metadata(&staging) {
            Ok(_) => {
                validate_private_directory(&staging)?;
                fs::remove_dir_all(&staging).map_err(|_| ManagerError::StorageFailed)?;
                sync_directory(&self.root.join("staging"))?;
            }
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(_) => return Err(ManagerError::StorageFailed),
        }
        ensure_private_directory(&staging)?;
        if let Err(error) = checkpoint(LifecycleCheckpoint::StagingCreated, &self.root) {
            let _ = fs::remove_dir_all(&staging);
            return Err(error);
        }

        let result = self.finalize(
            Finalization {
                manifest,
                runner,
                installed: &installed,
                staging: &staging,
                state,
                previous_state,
                idempotent,
            },
            offline_smoke,
            post_promotion_check,
            &mut checkpoint,
        );
        if result.is_err() {
            let _ = fs::remove_dir_all(&staging);
        }
        result
    }

    /// Reverify and atomically restore the previous immutable set offline.
    ///
    /// # Errors
    ///
    /// Missing/corrupt previous state or lock contention leaves current state
    /// unchanged and returns a stable failure.
    pub fn rollback(&self) -> Result<(), ManagerError> {
        let _lifecycle_lock = ExclusiveLifecycleLock::try_acquire(&self.root)?;
        let state_path = self.root.join("state.json");
        let mut state = read_private_state(&state_path)?;
        let previous = state.references().1.ok_or(ManagerError::StateInvalid)?;
        verify_digest(&self.root, previous)?;
        state.rollback()?;
        atomic_write(&state_path, &state.to_json()?, 0o600)
    }

    fn create_root(&self) -> Result<(), ManagerError> {
        if self.root.exists() {
            StorageRoot::validate_existing(&self.root)?;
        } else {
            fs::create_dir_all(&self.root).map_err(|_| ManagerError::StorageFailed)?;
            fs::set_permissions(&self.root, fs::Permissions::from_mode(0o700))
                .map_err(|_| ManagerError::StorageFailed)?;
            StorageRoot::validate_existing(&self.root)?;
        }
        for child in ["objects", "sets", "staging"] {
            let path = self.root.join(child);
            ensure_private_directory(&path)?;
        }
        ensure_lock_files(&self.root)?;
        Ok(())
    }

    fn finalize<S, P, C>(
        &self,
        finalization: Finalization<'_>,
        offline_smoke: S,
        post_promotion_check: P,
        checkpoint: &mut C,
    ) -> Result<(), ManagerError>
    where
        S: FnOnce(&Path, &ProviderManifest) -> Result<(), ManagerError>,
        P: FnOnce(&Path, &str) -> Result<(), ManagerError>,
        C: FnMut(LifecycleCheckpoint, &Path) -> Result<(), ManagerError>,
    {
        let Finalization {
            manifest,
            runner,
            installed,
            staging,
            mut state,
            previous_state,
            idempotent,
        } = finalization;
        write_object(
            staging,
            manifest.runner().sha256(),
            manifest.runner().installed_name(),
            runner,
            0o700,
        )?;
        for (_, name, content) in installed {
            write_object(staging, &sha256(content), name, content, 0o600)?;
        }
        checkpoint(LifecycleCheckpoint::ObjectsStaged, &self.root)?;
        offline_smoke(staging, manifest)?;

        promote_staged_object(
            staging,
            &self.root,
            manifest.runner().sha256(),
            manifest.runner().installed_name(),
            runner,
            0o700,
        )?;
        for (_, name, content) in installed {
            promote_staged_object(staging, &self.root, &sha256(content), name, content, 0o600)?;
        }
        checkpoint(LifecycleCheckpoint::ObjectsFinalized, &self.root)?;

        let runner_json = json!({
            "role": "runner",
            "object_digest": manifest.runner().sha256(),
            "installed_name": manifest.runner().installed_name(),
            "installed_size": manifest.runner().size(),
        });
        let artifact_json = installed
            .iter()
            .map(|(role, name, content)| {
                json!({
                    "role": role,
                    "object_digest": sha256(content),
                    "installed_name": name,
                    "installed_size": content.len(),
                })
            })
            .collect::<Vec<_>>();
        let set = json!({
            "schema_version": 1,
            "manifest_digest": manifest.artifact_set_digest(),
            "profile_id": manifest.profile_id(),
            "runner": runner_json,
            "artifacts": artifact_json,
            "verification_state": "verified",
            "offline_smoke": "passed",
            "resource_gate": "passed",
            "license_gate": "complete",
        });
        atomic_write(
            &self
                .root
                .join("sets")
                .join(format!("{}.json", manifest.artifact_set_digest())),
            &serde_json::to_vec(&set).map_err(|_| ManagerError::StateInvalid)?,
            0o600,
        )?;
        checkpoint(LifecycleCheckpoint::SetFinalized, &self.root)?;
        fs::remove_dir_all(staging).map_err(|_| ManagerError::StorageFailed)?;
        sync_directory(&self.root.join("staging"))?;

        if idempotent {
            return post_promotion_check(&self.root, manifest.artifact_set_digest());
        }

        let state_path = self.root.join("state.json");
        state.stage_candidate(manifest.artifact_set_digest())?;
        atomic_write(&state_path, &state.to_json()?, 0o600)?;
        checkpoint(LifecycleCheckpoint::CandidatePersisted, &self.root)?;
        state.promote_candidate()?;
        atomic_write(&state_path, &state.to_json()?, 0o600)?;
        if let Err(error) = post_promotion_check(&self.root, manifest.artifact_set_digest()) {
            restore_state(&state_path, previous_state.as_deref())?;
            return Err(error);
        }
        Ok(())
    }
}

fn production_offline_smoke(root: &Path, manifest: &ProviderManifest) -> Result<(), ManagerError> {
    let executable = root
        .join("objects")
        .join(manifest.runner().sha256())
        .join(manifest.runner().installed_name());
    let mut arguments = Vec::with_capacity(6);
    for (role, flag) in [
        ("model", "--model"),
        ("vocabulary", "--vocabulary"),
        ("lexical_shortlist", "--lexical-shortlist"),
    ] {
        let artifact = manifest
            .artifacts()
            .iter()
            .find(|artifact| artifact.role() == role)
            .ok_or(ManagerError::ManifestInvalid)?;
        arguments.push(flag.to_string());
        arguments.push(format!(
            "objects/{}/{}",
            artifact.installed_sha256(),
            artifact.installed_name()
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

fn read_private_state(path: &Path) -> Result<InstallationState, ManagerError> {
    parse_state(&read_private_file(path)?)
}

fn read_private_file(path: &Path) -> Result<Vec<u8>, ManagerError> {
    validate_private_file(path)?;
    fs::read(path).map_err(|_| ManagerError::StateInvalid)
}

fn parse_state(content: &[u8]) -> Result<InstallationState, ManagerError> {
    let input = std::str::from_utf8(content).map_err(|_| ManagerError::StateInvalid)?;
    InstallationState::from_json(input)
}

fn validate_runner(manifest: &ProviderManifest, runner: &[u8]) -> Result<(), ManagerError> {
    if usize::try_from(manifest.runner().size()).ok() != Some(runner.len())
        || sha256(runner) != manifest.runner().sha256()
    {
        return Err(ManagerError::IntegrityFailed);
    }
    Ok(())
}

fn validate_artifacts(
    artifacts: &[ModelArtifact],
    sources: &[ControlledArtifact],
) -> Result<Vec<(String, String, Vec<u8>)>, ManagerError> {
    if artifacts.len() != 3 || sources.len() != artifacts.len() {
        return Err(ManagerError::ManifestInvalid);
    }
    let source_by_role = sources
        .iter()
        .map(|source| (source.role.as_str(), source.compressed.as_slice()))
        .collect::<HashMap<_, _>>();
    if source_by_role.len() != sources.len() {
        return Err(ManagerError::ManifestInvalid);
    }
    artifacts
        .iter()
        .map(|artifact| {
            let compressed = source_by_role
                .get(artifact.role())
                .ok_or(ManagerError::ManifestInvalid)?;
            let compressed_size = usize::try_from(artifact.compressed_size())
                .map_err(|_| ManagerError::ManifestInvalid)?;
            let policy = AcquisitionPolicy::new(
                artifact.attachment_url(),
                compressed_size,
                artifact.compressed_sha256(),
            )?;
            let verified = policy.verify_reader(Cursor::new(compressed), Some(compressed.len()))?;
            let installed_size = usize::try_from(artifact.installed_size())
                .map_err(|_| ManagerError::ManifestInvalid)?;
            let expanded = expand_zstandard(
                Cursor::new(verified),
                installed_size,
                artifact.installed_sha256(),
            )?;
            Ok((
                artifact.role().to_string(),
                artifact.installed_name().to_string(),
                expanded,
            ))
        })
        .collect()
}

fn write_object(
    root: &Path,
    digest: &str,
    name: &str,
    content: &[u8],
    mode: u32,
) -> Result<(), ManagerError> {
    if sha256(content) != digest || !is_safe_basename(name) {
        return Err(ManagerError::IntegrityFailed);
    }
    ensure_private_directory(&root.join("objects"))?;
    let directory = root.join("objects").join(digest);
    ensure_private_directory(&directory)?;
    let path = directory.join(name);
    if path.exists() {
        let metadata = fs::symlink_metadata(&path).map_err(|_| ManagerError::StorageFailed)?;
        // SAFETY: `geteuid` has no arguments and no safety preconditions.
        let effective_uid = unsafe { libc::geteuid() };
        if metadata.file_type().is_symlink()
            || !metadata.is_file()
            || metadata.uid() != effective_uid
            || metadata.nlink() != 1
            || metadata.permissions().mode() & 0o077 != 0
        {
            return Err(ManagerError::StorageUnsafe);
        }
        let existing = fs::read(&path).map_err(|_| ManagerError::StorageFailed)?;
        if existing != content {
            return Err(ManagerError::IntegrityFailed);
        }
        return Ok(());
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(&path)
        .map_err(|_| ManagerError::StorageFailed)?;
    file.set_permissions(fs::Permissions::from_mode(mode))
        .map_err(|_| ManagerError::StorageFailed)?;
    file.write_all(content)
        .map_err(|_| ManagerError::StorageFailed)?;
    file.sync_all().map_err(|_| ManagerError::StorageFailed)?;
    sync_directory(&directory)
}

fn promote_staged_object(
    staging: &Path,
    root: &Path,
    digest: &str,
    name: &str,
    content: &[u8],
    mode: u32,
) -> Result<(), ManagerError> {
    let staged = staging.join("objects").join(digest);
    let finalized = root.join("objects").join(digest);
    match fs::symlink_metadata(&finalized) {
        Ok(_) => return write_object(root, digest, name, content, mode),
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(_) => return Err(ManagerError::StorageFailed),
    }
    validate_private_directory(&staged)?;
    fs::rename(staged, finalized).map_err(|_| ManagerError::StorageFailed)?;
    sync_directory(&root.join("objects"))
}

fn ensure_private_directory(path: &Path) -> Result<(), ManagerError> {
    match fs::symlink_metadata(path) {
        Ok(_) => validate_private_directory(path).map(|_| ()),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            fs::create_dir(path).map_err(|_| ManagerError::StorageFailed)?;
            fs::set_permissions(path, fs::Permissions::from_mode(0o700))
                .map_err(|_| ManagerError::StorageFailed)?;
            validate_private_directory(path)?;
            sync_directory(path.parent().ok_or(ManagerError::StorageFailed)?)
        }
        Err(_) => Err(ManagerError::StorageFailed),
    }
}

fn sync_directory(path: &Path) -> Result<(), ManagerError> {
    File::open(path)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| ManagerError::StorageFailed)
}

fn atomic_write(path: &Path, content: &[u8], mode: u32) -> Result<(), ManagerError> {
    let temporary = path.with_extension(format!("new-{}", std::process::id()));
    let result = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
            .open(&temporary)
            .map_err(|_| ManagerError::StorageUnsafe)?;
        file.set_permissions(fs::Permissions::from_mode(mode))
            .map_err(|_| ManagerError::StorageFailed)?;
        file.write_all(content)
            .map_err(|_| ManagerError::StorageFailed)?;
        file.sync_all().map_err(|_| ManagerError::StorageFailed)?;
        fs::rename(&temporary, path).map_err(|_| ManagerError::StorageFailed)?;
        sync_directory(path.parent().ok_or(ManagerError::StorageFailed)?)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}

fn restore_state(path: &Path, previous: Option<&[u8]>) -> Result<(), ManagerError> {
    if let Some(content) = previous {
        atomic_write(path, content, 0o600)
    } else {
        fs::remove_file(path).map_err(|_| ManagerError::StorageFailed)?;
        File::open(path.parent().ok_or(ManagerError::StorageFailed)?)
            .and_then(|directory| directory.sync_all())
            .map_err(|_| ManagerError::StorageFailed)
    }
}

fn sha256(content: &[u8]) -> String {
    Sha256::digest(content)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn is_safe_basename(value: &str) -> bool {
    !value.is_empty()
        && value != "."
        && value != ".."
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

fn ensure_free_space(path: &Path, required: u64) -> Result<(), ManagerError> {
    let existing = path
        .ancestors()
        .find(|ancestor| ancestor.exists())
        .ok_or(ManagerError::StorageFailed)?;
    let encoded = std::ffi::CString::new(existing.as_os_str().as_bytes())
        .map_err(|_| ManagerError::StorageFailed)?;
    let mut stats = std::mem::MaybeUninit::<libc::statvfs>::uninit();
    // SAFETY: `encoded` is NUL-terminated and `stats` points to writable memory.
    if unsafe { libc::statvfs(encoded.as_ptr(), stats.as_mut_ptr()) } != 0 {
        return Err(ManagerError::StorageFailed);
    }
    // SAFETY: a successful `statvfs` call initialized the structure.
    let stats = unsafe { stats.assume_init() };
    let available = stats.f_bavail.saturating_mul(stats.f_frsize);
    if available < required {
        return Err(ManagerError::StorageFailed);
    }
    Ok(())
}
