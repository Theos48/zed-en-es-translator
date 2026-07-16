use std::fs::{File, OpenOptions};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::Path;

use crate::error::ManagerError;

/// Held exclusive serialization lock for lifecycle mutations.
#[derive(Debug)]
pub struct ExclusiveLifecycleLock {
    _file: File,
}

/// Held shared lock for a bounded state/set resolution snapshot.
#[derive(Debug)]
pub struct SharedStateLock {
    _file: File,
}

impl SharedStateLock {
    /// Try to read lifecycle state without racing an atomic promotion.
    ///
    /// # Errors
    ///
    /// Returns `BUSY` while a lifecycle mutation owns the exclusive lock.
    pub fn try_acquire(root: &Path) -> Result<Self, ManagerError> {
        let file = private_lock_file(&root.join("lifecycle.lock"))?;
        fs4::FileExt::try_lock_shared(&file).map_err(|_| ManagerError::Busy)?;
        Ok(Self { _file: file })
    }
}

impl ExclusiveLifecycleLock {
    /// Try to serialize one lifecycle operation without waiting indefinitely.
    ///
    /// # Errors
    ///
    /// Returns `BUSY` when another operation owns the lock.
    pub fn try_acquire(root: &Path) -> Result<Self, ManagerError> {
        let file = private_lock_file(&root.join("lifecycle.lock"))?;
        fs4::FileExt::try_lock(&file).map_err(|_| ManagerError::Busy)?;
        Ok(Self { _file: file })
    }
}

/// Held shared lease proving one inference may use immutable objects.
#[derive(Debug)]
pub struct SharedInferenceLease {
    _file: File,
    path: std::path::PathBuf,
}

impl SharedInferenceLease {
    /// Try to acquire the shared provider-object lease.
    ///
    /// # Errors
    ///
    /// Returns `BUSY` while cleanup owns the exclusive lease.
    pub fn try_acquire(root: &Path) -> Result<Self, ManagerError> {
        let file = private_lock_file(&root.join("lease.lock"))?;
        fs4::FileExt::try_lock_shared(&file).map_err(|_| ManagerError::Busy)?;
        Ok(Self {
            _file: file,
            path: root.join("lease.lock"),
        })
    }

    /// Test the exclusive peer while this shared lease remains held.
    ///
    /// # Errors
    ///
    /// Always returns `BUSY` for a functioning advisory-lock backend.
    pub fn try_exclusive_peer(&self) -> Result<ExclusiveInferenceLease, ManagerError> {
        let peer = private_lock_file(&self.path)?;
        fs4::FileExt::try_lock(&peer).map_err(|_| ManagerError::Busy)?;
        Ok(ExclusiveInferenceLease { _file: peer })
    }
}

/// Held exclusive lease used only for complete removal.
#[derive(Debug)]
pub struct ExclusiveInferenceLease {
    _file: File,
}

impl ExclusiveInferenceLease {
    /// Try to block every active/new inference before removal.
    ///
    /// # Errors
    ///
    /// Returns `BUSY` while any shared inference lease exists.
    pub fn try_acquire(root: &Path) -> Result<Self, ManagerError> {
        let file = private_lock_file(&root.join("lease.lock"))?;
        fs4::FileExt::try_lock(&file).map_err(|_| ManagerError::Busy)?;
        Ok(Self { _file: file })
    }
}

/// Materialize both lifecycle lock files during successful root setup.
pub(crate) fn ensure_lock_files(root: &Path) -> Result<(), ManagerError> {
    private_lock_file(&root.join("lifecycle.lock"))?;
    private_lock_file(&root.join("lease.lock"))?;
    Ok(())
}

fn private_lock_file(path: &Path) -> Result<File, ManagerError> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)
        .open(path)
        .map_err(|_| ManagerError::StorageUnsafe)?;
    let metadata = file.metadata().map_err(|_| ManagerError::StorageFailed)?;
    // SAFETY: `geteuid` has no arguments and no safety preconditions.
    let effective_uid = unsafe { libc::geteuid() };
    if !metadata.is_file()
        || metadata.uid() != effective_uid
        || metadata.nlink() != 1
        || metadata.permissions().mode() & 0o077 != 0
    {
        return Err(ManagerError::StorageUnsafe);
    }
    Ok(file)
}
