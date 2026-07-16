use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use crate::error::ManagerError;
use crate::locking::{ExclusiveInferenceLease, ExclusiveLifecycleLock};
use crate::storage::StorageRoot;

const CONFIRMATION: &str = "remove-embedded-provider-data";
const KNOWN_ENTRIES: &[&str] = &[
    "lifecycle.lock",
    "lease.lock",
    "objects",
    "sets",
    "staging",
    "state.json",
];

/// Remove only the complete, recognized provider-owned root.
///
/// # Errors
///
/// Wrong confirmation, unsafe links, unknown entries, or active inference
/// leases fail before deletion.
pub fn clean(root: &Path, confirmation: &str) -> Result<(), ManagerError> {
    if confirmation != CONFIRMATION {
        return Err(ManagerError::ConsentRequired);
    }
    if !root.exists() {
        return Ok(());
    }
    StorageRoot::validate_existing(root)?;
    validate_owned_tree(root)?;
    let _lifecycle = ExclusiveLifecycleLock::try_acquire(root)?;
    let _lease = ExclusiveInferenceLease::try_acquire(root)?;
    validate_owned_tree(root)?;
    for entry in fs::read_dir(root).map_err(|_| ManagerError::StorageFailed)? {
        let entry = entry.map_err(|_| ManagerError::StorageFailed)?;
        let metadata =
            fs::symlink_metadata(entry.path()).map_err(|_| ManagerError::StorageFailed)?;
        if metadata.is_dir() {
            fs::remove_dir_all(entry.path()).map_err(|_| ManagerError::StorageFailed)?;
        } else {
            fs::remove_file(entry.path()).map_err(|_| ManagerError::StorageFailed)?;
        }
    }
    fs::remove_dir(root).map_err(|_| ManagerError::StorageFailed)
}

fn validate_owned_tree(root: &Path) -> Result<(), ManagerError> {
    for entry in fs::read_dir(root).map_err(|_| ManagerError::StorageFailed)? {
        let entry = entry.map_err(|_| ManagerError::StorageFailed)?;
        let name = entry.file_name();
        let name = name.to_str().ok_or(ManagerError::CleanupRefused)?;
        if !KNOWN_ENTRIES.contains(&name) {
            return Err(ManagerError::CleanupRefused);
        }
        reject_links_recursively(&entry.path())?;
    }
    Ok(())
}

fn reject_links_recursively(path: &Path) -> Result<(), ManagerError> {
    let metadata = fs::symlink_metadata(path).map_err(|_| ManagerError::CleanupRefused)?;
    if metadata.file_type().is_symlink() {
        return Err(ManagerError::CleanupRefused);
    }
    if metadata.is_file() && metadata.nlink() != 1 {
        return Err(ManagerError::CleanupRefused);
    }
    if metadata.is_dir() {
        for entry in fs::read_dir(path).map_err(|_| ManagerError::CleanupRefused)? {
            reject_links_recursively(&entry.map_err(|_| ManagerError::CleanupRefused)?.path())?;
        }
    }
    Ok(())
}
