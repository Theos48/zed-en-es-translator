use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use ruzstd::decoding::StreamingDecoder;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use zed_extension_api as zed;

use crate::package::{InstalledPackage, ModelResource, PublishedPackage, ServerFile};

const STATE_BYTES_LIMIT: u64 = 16 * 1024;

/// Download/extraction behavior requested from Zed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadKind {
    /// Extract a project release `.tar.gz` into the staging directory.
    GzipTar,
    /// Preserve one compressed model resource exactly as downloaded.
    Uncompressed,
}

/// Narrow downloader boundary used by the real Zed host and controlled tests.
pub trait PackageDownloader {
    fn download(&mut self, url: &str, destination: &Path, kind: DownloadKind)
        -> Result<(), String>;

    fn make_executable(&mut self, path: &Path) -> Result<(), String>;
}

/// Zed-host implementation of the fixed download boundary.
#[derive(Debug, Default, Clone, Copy)]
pub struct ZedDownloader;

impl PackageDownloader for ZedDownloader {
    fn download(
        &mut self,
        url: &str,
        destination: &Path,
        kind: DownloadKind,
    ) -> Result<(), String> {
        let destination = destination
            .to_str()
            .ok_or_else(|| "The package destination is invalid.".to_string())?;
        let file_type = match kind {
            DownloadKind::GzipTar => zed::DownloadedFileType::GzipTar,
            DownloadKind::Uncompressed => zed::DownloadedFileType::Uncompressed,
        };
        zed::download_file(url, destination, file_type)
            .map_err(|_| "The translation package download failed.".to_string())
    }

    fn make_executable(&mut self, path: &Path) -> Result<(), String> {
        let path = path
            .to_str()
            .ok_or_else(|| "The package executable path is invalid.".to_string())?;
        zed::make_file_executable(path)
            .map_err(|_| "The translation package could not be activated.".to_string())
    }
}

/// Platform decision evaluated before any downloader call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostPlatform {
    LinuxX8664,
    LinuxAarch64,
    LinuxX86,
    MacAarch64,
    MacX86,
    MacX8664,
    WindowsAarch64,
    WindowsX86,
    WindowsX8664,
}

/// Stable, content-free acquisition error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcquisitionError {
    UnsupportedPlatform,
    Busy,
    DownloadFailed,
    InvalidPackage,
    StorageFailed,
}

impl fmt::Display for AcquisitionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::UnsupportedPlatform => {
                "Automatic local translation currently supports Linux x86_64 only."
            }
            Self::Busy => "The translation package is already being prepared.",
            Self::DownloadFailed => "The translation package download failed. Retry in Zed.",
            Self::InvalidPackage => "The downloaded translation package failed verification.",
            Self::StorageFailed => "Zed could not store the verified translation package.",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for AcquisitionError {}

/// Automatic package preparation rooted only in Zed's extension work directory.
pub struct Acquisition<D> {
    root: PathBuf,
    downloader: D,
}

impl<D: PackageDownloader> Acquisition<D> {
    pub fn new(root: impl Into<PathBuf>, downloader: D) -> Self {
        Self {
            root: root.into(),
            downloader,
        }
    }

    pub fn downloader(&self) -> &D {
        &self.downloader
    }

    /// Check whether the exact active package is ready without creating state
    /// or entering the download path.
    pub fn ready_command(
        &self,
        package: &PublishedPackage,
        platform: HostPlatform,
    ) -> Result<Option<PathBuf>, AcquisitionError> {
        if platform != HostPlatform::LinuxX8664 {
            return Err(AcquisitionError::UnsupportedPlatform);
        }
        if !self.root.exists() {
            return Ok(None);
        }
        Ok(verified_command(&self.root, package).ok())
    }

    /// Return a verified server path, preparing the fixed package if needed.
    pub fn prepare(
        &mut self,
        package: &PublishedPackage,
        platform: HostPlatform,
    ) -> Result<PathBuf, AcquisitionError> {
        if platform != HostPlatform::LinuxX8664 {
            return Err(AcquisitionError::UnsupportedPlatform);
        }
        create_root(&self.root)?;
        if let Ok(command) = verified_command(&self.root, package) {
            return Ok(command);
        }

        let _lock = PreparationLock::acquire(&self.root)?;
        if let Ok(command) = verified_command(&self.root, package) {
            return Ok(command);
        }
        self.prepare_locked(package)
    }

    fn prepare_locked(&mut self, package: &PublishedPackage) -> Result<PathBuf, AcquisitionError> {
        let staging_parent = self.root.join("staging");
        let staging = staging_parent.join(package.package_id());
        fs::create_dir_all(&staging_parent).map_err(|_| AcquisitionError::StorageFailed)?;
        remove_directory_if_present(&staging)?;
        fs::create_dir(&staging).map_err(|_| AcquisitionError::StorageFailed)?;

        let result = self.populate_staging(package, &staging).and_then(|()| {
            let packages = self.root.join("packages");
            fs::create_dir_all(&packages).map_err(|_| AcquisitionError::StorageFailed)?;
            let installed = packages.join(package.package_id());
            remove_directory_if_present(&installed)?;
            fs::rename(&staging, &installed).map_err(|_| AcquisitionError::StorageFailed)?;
            promote_state(&self.root, package.package_id())?;
            verified_command(&self.root, package)
        });
        if result.is_err() {
            let _ = remove_directory_if_present(&staging);
        }
        match result {
            Ok(command) => Ok(command),
            Err(error) => last_known_good_command(&self.root).or(Err(error)),
        }
    }

    fn populate_staging(
        &mut self,
        package: &PublishedPackage,
        staging: &Path,
    ) -> Result<(), AcquisitionError> {
        self.downloader
            .download(package.server_archive_url(), staging, DownloadKind::GzipTar)
            .map_err(|_| AcquisitionError::DownloadFailed)?;
        verify_server_archive(staging, package.server_files())?;
        for file in package
            .server_files()
            .iter()
            .filter(|file| file.executable())
        {
            self.downloader
                .make_executable(&staging.join(file.path()))
                .map_err(|_| AcquisitionError::StorageFailed)?;
        }

        let downloads = staging.join(".downloads");
        let models = staging.join("models");
        fs::create_dir(&downloads).map_err(|_| AcquisitionError::StorageFailed)?;
        fs::create_dir(&models).map_err(|_| AcquisitionError::StorageFailed)?;
        for resource in package.model_resources() {
            self.install_model(resource, &downloads, &models)?;
        }
        fs::remove_dir(&downloads).map_err(|_| AcquisitionError::StorageFailed)?;

        let manifest = package
            .installed_manifest()
            .to_json()
            .map_err(|_| AcquisitionError::InvalidPackage)?;
        write_new(&staging.join("installed.json"), &manifest)?;
        verify_staging_tree(staging, package)
    }

    fn install_model(
        &mut self,
        resource: &ModelResource,
        downloads: &Path,
        models: &Path,
    ) -> Result<(), AcquisitionError> {
        let compressed = downloads.join(resource.compressed_name());
        self.downloader
            .download(resource.url(), &compressed, DownloadKind::Uncompressed)
            .map_err(|_| AcquisitionError::DownloadFailed)?;
        verify_file(
            &compressed,
            resource.compressed_size(),
            resource.compressed_sha256(),
        )?;
        let installed = models.join(resource.installed_name());
        decode_zstandard(
            &compressed,
            &installed,
            resource.installed_size(),
            resource.installed_sha256(),
        )?;
        fs::remove_file(compressed).map_err(|_| AcquisitionError::StorageFailed)
    }
}

fn create_root(root: &Path) -> Result<(), AcquisitionError> {
    fs::create_dir_all(root).map_err(|_| AcquisitionError::StorageFailed)?;
    let metadata = fs::symlink_metadata(root).map_err(|_| AcquisitionError::StorageFailed)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(AcquisitionError::StorageFailed);
    }
    Ok(())
}

fn verified_command(root: &Path, package: &PublishedPackage) -> Result<PathBuf, AcquisitionError> {
    let package_root = root.join("packages").join(package.package_id());
    verify_staging_tree(&package_root, package)?;
    let state = read_state(root)?;
    if state.active.as_deref() != Some(package.package_id()) {
        return Err(AcquisitionError::InvalidPackage);
    }
    Ok(package_root.join("bin/translator-lsp"))
}

fn last_known_good_command(root: &Path) -> Result<PathBuf, AcquisitionError> {
    let state = read_state(root)?;
    let package_id = state.active.ok_or(AcquisitionError::InvalidPackage)?;
    let package_root = root.join("packages").join(&package_id);
    let metadata =
        fs::symlink_metadata(&package_root).map_err(|_| AcquisitionError::InvalidPackage)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(AcquisitionError::InvalidPackage);
    }
    let manifest_path = package_root.join("installed.json");
    let manifest_metadata =
        fs::symlink_metadata(&manifest_path).map_err(|_| AcquisitionError::InvalidPackage)?;
    if manifest_metadata.file_type().is_symlink()
        || !manifest_metadata.is_file()
        || manifest_metadata.len() == 0
        || manifest_metadata.len() > STATE_BYTES_LIMIT
    {
        return Err(AcquisitionError::InvalidPackage);
    }
    let manifest = InstalledPackage::parse(
        &fs::read(manifest_path).map_err(|_| AcquisitionError::InvalidPackage)?,
    )
    .map_err(|_| AcquisitionError::InvalidPackage)?;
    if manifest.package_id != package_id {
        return Err(AcquisitionError::InvalidPackage);
    }
    for artifact in &manifest.artifacts {
        verify_file(
            &package_root.join(&artifact.path),
            artifact.installed_size,
            &artifact.installed_sha256,
        )?;
    }
    Ok(package_root.join("bin/translator-lsp"))
}

fn verify_server_archive(root: &Path, expected: &[ServerFile]) -> Result<(), AcquisitionError> {
    let actual = collect_files(root)?;
    let allowed = expected
        .iter()
        .map(|file| file.path().to_string())
        .collect::<BTreeSet<_>>();
    if actual != allowed {
        return Err(AcquisitionError::InvalidPackage);
    }
    for file in expected {
        verify_file(&root.join(file.path()), file.size(), file.sha256())?;
    }
    Ok(())
}

fn verify_staging_tree(root: &Path, package: &PublishedPackage) -> Result<(), AcquisitionError> {
    let metadata = fs::symlink_metadata(root).map_err(|_| AcquisitionError::InvalidPackage)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(AcquisitionError::InvalidPackage);
    }
    let mut allowed = package
        .server_files()
        .iter()
        .map(|file| file.path().to_string())
        .collect::<BTreeSet<_>>();
    for resource in package.model_resources() {
        allowed.insert(format!("models/{}", resource.installed_name()));
        verify_file(
            &root.join("models").join(resource.installed_name()),
            resource.installed_size(),
            resource.installed_sha256(),
        )?;
    }
    allowed.insert("installed.json".to_string());
    if collect_files(root)? != allowed {
        return Err(AcquisitionError::InvalidPackage);
    }
    for file in package.server_files() {
        verify_file(&root.join(file.path()), file.size(), file.sha256())?;
    }
    let expected_manifest = package
        .installed_manifest()
        .to_json()
        .map_err(|_| AcquisitionError::InvalidPackage)?;
    let manifest =
        fs::read(root.join("installed.json")).map_err(|_| AcquisitionError::InvalidPackage)?;
    if manifest != expected_manifest {
        return Err(AcquisitionError::InvalidPackage);
    }
    Ok(())
}

fn collect_files(root: &Path) -> Result<BTreeSet<String>, AcquisitionError> {
    let mut pending = vec![root.to_path_buf()];
    let mut files = BTreeSet::new();
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory).map_err(|_| AcquisitionError::InvalidPackage)? {
            let entry = entry.map_err(|_| AcquisitionError::InvalidPackage)?;
            let file_type = entry
                .file_type()
                .map_err(|_| AcquisitionError::InvalidPackage)?;
            if file_type.is_symlink() {
                return Err(AcquisitionError::InvalidPackage);
            }
            let path = entry.path();
            if file_type.is_dir() {
                pending.push(path);
            } else if file_type.is_file() {
                let relative = path
                    .strip_prefix(root)
                    .ok()
                    .and_then(Path::to_str)
                    .ok_or(AcquisitionError::InvalidPackage)?;
                files.insert(relative.to_string());
            } else {
                return Err(AcquisitionError::InvalidPackage);
            }
        }
    }
    Ok(files)
}

fn verify_file(path: &Path, size: u64, sha256: &str) -> Result<(), AcquisitionError> {
    let metadata = fs::symlink_metadata(path).map_err(|_| AcquisitionError::InvalidPackage)?;
    if metadata.file_type().is_symlink() || !metadata.is_file() || metadata.len() != size {
        return Err(AcquisitionError::InvalidPackage);
    }
    let mut file = File::open(path).map_err(|_| AcquisitionError::InvalidPackage)?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 32 * 1024];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|_| AcquisitionError::InvalidPackage)?;
        if count == 0 {
            break;
        }
        digest.update(&buffer[..count]);
    }
    if hex_digest(digest.finalize().as_slice()) != sha256 {
        return Err(AcquisitionError::InvalidPackage);
    }
    Ok(())
}

fn decode_zstandard(
    compressed: &Path,
    installed: &Path,
    expected_size: u64,
    expected_sha256: &str,
) -> Result<(), AcquisitionError> {
    let source = File::open(compressed).map_err(|_| AcquisitionError::InvalidPackage)?;
    let mut decoder =
        StreamingDecoder::new(source).map_err(|_| AcquisitionError::InvalidPackage)?;
    let mut destination = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(installed)
        .map_err(|_| AcquisitionError::StorageFailed)?;
    let mut digest = Sha256::new();
    let mut written = 0_u64;
    let mut buffer = [0_u8; 32 * 1024];
    loop {
        let count = decoder
            .read(&mut buffer)
            .map_err(|_| AcquisitionError::InvalidPackage)?;
        if count == 0 {
            break;
        }
        written = written
            .checked_add(count as u64)
            .ok_or(AcquisitionError::InvalidPackage)?;
        if written > expected_size {
            return Err(AcquisitionError::InvalidPackage);
        }
        digest.update(&buffer[..count]);
        destination
            .write_all(&buffer[..count])
            .map_err(|_| AcquisitionError::StorageFailed)?;
    }
    destination
        .flush()
        .map_err(|_| AcquisitionError::StorageFailed)?;
    if written != expected_size || hex_digest(digest.finalize().as_slice()) != expected_sha256 {
        return Err(AcquisitionError::InvalidPackage);
    }
    Ok(())
}

fn hex_digest(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn write_new(path: &Path, contents: &[u8]) -> Result<(), AcquisitionError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|_| AcquisitionError::StorageFailed)?;
    file.write_all(contents)
        .and_then(|()| file.flush())
        .map_err(|_| AcquisitionError::StorageFailed)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PackageState {
    schema_version: u32,
    generation: u64,
    active: Option<String>,
    previous: Option<String>,
}

fn read_state(root: &Path) -> Result<PackageState, AcquisitionError> {
    let path = root.join("state.json");
    let metadata = fs::symlink_metadata(&path).map_err(|_| AcquisitionError::InvalidPackage)?;
    if metadata.file_type().is_symlink()
        || !metadata.is_file()
        || metadata.len() == 0
        || metadata.len() > STATE_BYTES_LIMIT
    {
        return Err(AcquisitionError::InvalidPackage);
    }
    let state: PackageState =
        serde_json::from_slice(&fs::read(path).map_err(|_| AcquisitionError::InvalidPackage)?)
            .map_err(|_| AcquisitionError::InvalidPackage)?;
    if state.schema_version != 1
        || state.active.as_ref().is_none_or(|value| !is_safe_id(value))
        || state
            .previous
            .as_ref()
            .is_some_and(|value| !is_safe_id(value))
        || state.active == state.previous
    {
        return Err(AcquisitionError::InvalidPackage);
    }
    Ok(state)
}

fn promote_state(root: &Path, package_id: &str) -> Result<(), AcquisitionError> {
    let current = read_state(root).ok();
    let previous = current
        .as_ref()
        .and_then(|state| state.active.clone())
        .filter(|active| active != package_id);
    let state = PackageState {
        schema_version: 1,
        generation: current.map_or(1, |state| state.generation.saturating_add(1)),
        active: Some(package_id.to_string()),
        previous,
    };
    let contents =
        serde_json::to_vec_pretty(&state).map_err(|_| AcquisitionError::StorageFailed)?;
    prune_unreferenced_packages(root, package_id, state.previous.as_deref())?;
    let next = root.join("state.next.json");
    let _ = fs::remove_file(&next);
    write_new(&next, &contents)?;
    fs::rename(next, root.join("state.json")).map_err(|_| AcquisitionError::StorageFailed)
}

fn prune_unreferenced_packages(
    root: &Path,
    active: &str,
    previous: Option<&str>,
) -> Result<(), AcquisitionError> {
    let packages = root.join("packages");
    for entry in fs::read_dir(packages).map_err(|_| AcquisitionError::StorageFailed)? {
        let entry = entry.map_err(|_| AcquisitionError::StorageFailed)?;
        let name = entry.file_name();
        if name.as_os_str() == OsStr::new(active)
            || previous.is_some_and(|previous| name.as_os_str() == OsStr::new(previous))
        {
            continue;
        }
        let file_type = entry
            .file_type()
            .map_err(|_| AcquisitionError::StorageFailed)?;
        if file_type.is_dir() {
            fs::remove_dir_all(entry.path()).map_err(|_| AcquisitionError::StorageFailed)?;
        } else {
            fs::remove_file(entry.path()).map_err(|_| AcquisitionError::StorageFailed)?;
        }
    }
    Ok(())
}

fn is_safe_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"._-".contains(&byte)
        })
}

fn remove_directory_if_present(path: &Path) -> Result<(), AcquisitionError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            Err(AcquisitionError::StorageFailed)
        }
        Ok(_) => fs::remove_dir_all(path).map_err(|_| AcquisitionError::StorageFailed),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(_) => Err(AcquisitionError::StorageFailed),
    }
}

struct PreparationLock {
    path: PathBuf,
}

impl PreparationLock {
    fn acquire(root: &Path) -> Result<Self, AcquisitionError> {
        let path = root.join("install.lock");
        match create_lock_file(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                if !is_stale_lock(&path) {
                    return Err(AcquisitionError::Busy);
                }
                fs::remove_file(&path).map_err(|_| AcquisitionError::Busy)?;
                create_lock_file(&path).map_err(|error| {
                    if error.kind() == std::io::ErrorKind::AlreadyExists {
                        AcquisitionError::Busy
                    } else {
                        AcquisitionError::StorageFailed
                    }
                })?;
            }
            Err(_) => return Err(AcquisitionError::StorageFailed),
        }
        Ok(Self { path })
    }
}

fn create_lock_file(path: &Path) -> std::io::Result<()> {
    let created = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs());
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .and_then(|mut file| write!(file, "schema_version=1\ncreated_unix_seconds={created}\n"))
}

fn is_stale_lock(path: &Path) -> bool {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return false;
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() || metadata.len() > 128 {
        return false;
    }
    let Ok(contents) = fs::read_to_string(path) else {
        return false;
    };
    let mut lines = contents.lines();
    if lines.next() != Some("schema_version=1") {
        return false;
    }
    let Some(created) = lines
        .next()
        .and_then(|line| line.strip_prefix("created_unix_seconds="))
        .and_then(|value| value.parse::<u64>().ok())
    else {
        return false;
    };
    if lines.next().is_some() {
        return false;
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs());
    now.saturating_sub(created) > 15 * 60
}

impl Drop for PreparationLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}
