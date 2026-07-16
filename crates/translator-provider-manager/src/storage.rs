use std::fmt;
use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};

use crate::error::ManagerError;

const TMPFS_MAGIC: libc::c_long = 0x0102_1994;
const RAMFS_MAGIC: libc::c_long = 0x8584_58f6_u32 as libc::c_long;

/// Validated private persistent root owned by the current user.
pub struct StorageRoot {
    path: PathBuf,
}

impl StorageRoot {
    /// Derive the product-owned provider root from XDG data conventions.
    ///
    /// # Errors
    ///
    /// Returns a storage error when neither `XDG_DATA_HOME` nor `HOME` supplies
    /// a usable user-scoped base directory.
    pub fn fixed_path() -> Result<PathBuf, ManagerError> {
        let data_home = std::env::var_os("XDG_DATA_HOME")
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME")
                    .filter(|value| !value.is_empty())
                    .map(|home| PathBuf::from(home).join(".local/share"))
            })
            .ok_or(ManagerError::StorageFailed)?;
        Ok(data_home.join("zed-en-es-translator/embedded"))
    }

    /// Validate an existing provider root without following a root symlink.
    ///
    /// # Errors
    ///
    /// Returns a content-free error when the root is not a private directory
    /// owned by the current user on persistent storage.
    pub fn validate_existing(path: &Path) -> Result<Self, ManagerError> {
        let metadata = fs::symlink_metadata(path).map_err(|_| ManagerError::StorageFailed)?;
        if metadata.file_type().is_symlink()
            || !metadata.is_dir()
            || metadata.uid() != current_uid()
            || metadata.permissions().mode() & 0o077 != 0
            || is_volatile_filesystem(path)?
        {
            return Err(ManagerError::StorageUnsafe);
        }

        Ok(Self {
            path: path.to_path_buf(),
        })
    }

    /// Return the validated root path to internal lifecycle code.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl fmt::Debug for StorageRoot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StorageRoot")
            .finish_non_exhaustive()
    }
}

fn current_uid() -> u32 {
    // SAFETY: `geteuid` has no arguments and no safety preconditions.
    unsafe { libc::geteuid() }
}

fn is_volatile_filesystem(path: &Path) -> Result<bool, ManagerError> {
    let path = std::ffi::CString::new(path.as_os_str().as_encoded_bytes())
        .map_err(|_| ManagerError::StorageUnsafe)?;
    let mut stats = std::mem::MaybeUninit::<libc::statfs>::uninit();
    // SAFETY: `path` is NUL-terminated and `stats` points to writable storage.
    let result = unsafe { libc::statfs(path.as_ptr(), stats.as_mut_ptr()) };
    if result != 0 {
        return Err(ManagerError::StorageFailed);
    }
    // SAFETY: a zero return from `statfs` initializes the output structure.
    let stats = unsafe { stats.assume_init() };
    Ok(matches!(
        stats.f_type as libc::c_long,
        TMPFS_MAGIC | RAMFS_MAGIC
    ))
}
