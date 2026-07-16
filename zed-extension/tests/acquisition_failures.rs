mod common;

use std::fs;
use std::path::Path;

use sha2::{Digest as _, Sha256};
use zed_en_es_translator_extension::acquisition::{
    Acquisition, AcquisitionError, DownloadKind, HostPlatform, PackageDownloader,
};
use zed_en_es_translator_extension::package::PublishedPackage;

use common::{clean_root, test_lock_json, ControlledDownloader};

#[derive(Clone, Copy)]
enum Fault {
    Truncate,
    Oversize,
    Corrupt,
    InvalidZstd,
    ExecutableFailure,
}

struct FaultyDownloader {
    inner: ControlledDownloader,
    fault: Fault,
    applied: bool,
}

impl FaultyDownloader {
    fn new(fault: Fault) -> Self {
        Self {
            inner: ControlledDownloader::default(),
            fault,
            applied: false,
        }
    }
}

impl PackageDownloader for FaultyDownloader {
    fn download(
        &mut self,
        url: &str,
        destination: &Path,
        kind: DownloadKind,
    ) -> Result<(), String> {
        self.inner.download(url, destination, kind)?;
        if kind == DownloadKind::Uncompressed && !self.applied {
            self.applied = true;
            let mut contents = fs::read(destination).map_err(|_| "fault read".to_string())?;
            match self.fault {
                Fault::Truncate => {
                    contents.pop();
                }
                Fault::Oversize => contents.push(0),
                Fault::Corrupt => contents[0] ^= 0xff,
                Fault::InvalidZstd => contents = b"not-zstandard".to_vec(),
                Fault::ExecutableFailure => {}
            }
            fs::write(destination, contents).map_err(|_| "fault write".to_string())?;
        }
        Ok(())
    }

    fn make_executable(&mut self, path: &Path) -> Result<(), String> {
        if matches!(self.fault, Fault::ExecutableFailure) {
            return Err("controlled storage failure".to_string());
        }
        self.inner.make_executable(path)
    }
}

#[test]
fn truncation_oversize_and_hash_corruption_never_activate_a_package() {
    for (iteration, fault) in [Fault::Truncate, Fault::Oversize, Fault::Corrupt]
        .into_iter()
        .enumerate()
    {
        let root = clean_root("invalid-download", iteration);
        let _ = fs::remove_dir_all(&root);
        let package = PublishedPackage::parse(&test_lock_json(&format!("invalid-{iteration}")))
            .expect("controlled lock");
        let mut acquisition = Acquisition::new(&root, FaultyDownloader::new(fault));

        let error = acquisition
            .prepare(&package, HostPlatform::LinuxX8664)
            .expect_err("invalid download must fail");

        assert_eq!(error, AcquisitionError::InvalidPackage);
        assert!(!root.join("state.json").exists());
        assert!(!root.join("packages").join(package.package_id()).exists());
        assert!(!root.join("install.lock").exists());
        let _ = fs::remove_dir_all(root);
    }
}

#[test]
fn invalid_zstandard_data_fails_after_its_download_identity_verifies() {
    let root = clean_root("invalid-zstd", 0);
    let _ = fs::remove_dir_all(&root);
    let mut lock: serde_json::Value =
        serde_json::from_str(&test_lock_json("invalid-zstd")).expect("lock JSON");
    let bytes = b"not-zstandard";
    lock["model_resources"][0]["compressed_size"] = serde_json::json!(bytes.len());
    lock["model_resources"][0]["compressed_sha256"] = serde_json::json!(hex_digest(bytes));
    let package = PublishedPackage::parse(&serde_json::to_string(&lock).expect("lock"))
        .expect("controlled invalid-zstd lock");
    let mut acquisition = Acquisition::new(&root, FaultyDownloader::new(Fault::InvalidZstd));

    let error = acquisition
        .prepare(&package, HostPlatform::LinuxX8664)
        .expect_err("invalid zstd must fail");

    assert_eq!(error, AcquisitionError::InvalidPackage);
    assert!(!root.join("state.json").exists());
    let _ = fs::remove_dir_all(root);
}

#[test]
fn storage_failure_and_interrupted_staging_are_retry_safe() {
    let package =
        PublishedPackage::parse(&test_lock_json("storage-retry")).expect("controlled lock");
    let root = clean_root("storage", 0);
    let _ = fs::remove_dir_all(&root);
    let mut failed = Acquisition::new(&root, FaultyDownloader::new(Fault::ExecutableFailure));
    assert_eq!(
        failed
            .prepare(&package, HostPlatform::LinuxX8664)
            .expect_err("executable failure"),
        AcquisitionError::StorageFailed
    );
    drop(failed);

    let stale = root.join("staging").join(package.package_id());
    fs::create_dir_all(&stale).expect("stale staging");
    fs::write(stale.join("interrupted.partial"), b"partial").expect("partial staging");
    let mut retry = Acquisition::new(&root, ControlledDownloader::default());
    let command = retry
        .prepare(&package, HostPlatform::LinuxX8664)
        .expect("normal retry");

    assert!(command.is_file());
    assert!(!stale.exists());
    let _ = fs::remove_dir_all(root);

    let file_root = clean_root("root-is-file", 0);
    if let Some(parent) = file_root.parent() {
        fs::create_dir_all(parent).expect("file root parent");
    }
    fs::write(&file_root, b"not a directory").expect("file root");
    let mut storage = Acquisition::new(&file_root, ControlledDownloader::default());
    assert_eq!(
        storage
            .prepare(&package, HostPlatform::LinuxX8664)
            .expect_err("invalid root"),
        AcquisitionError::StorageFailed
    );
    fs::remove_file(file_root).expect("remove file root");
}

fn hex_digest(contents: &[u8]) -> String {
    Sha256::digest(contents)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
