mod common;

use std::fs;
use std::path::Path;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use zed_en_es_translator_extension::acquisition::{
    Acquisition, AcquisitionError, DownloadKind, HostPlatform, PackageDownloader,
};
use zed_en_es_translator_extension::package::PublishedPackage;

use common::{clean_root, test_lock_json, ControlledDownloader};

struct BlockingDownloader {
    inner: ControlledDownloader,
    gate: Arc<(Mutex<bool>, Condvar)>,
}

impl PackageDownloader for BlockingDownloader {
    fn download(
        &mut self,
        url: &str,
        destination: &Path,
        kind: DownloadKind,
    ) -> Result<(), String> {
        if kind == DownloadKind::GzipTar {
            let (lock, condition) = &*self.gate;
            let mut released = lock.lock().map_err(|_| "gate".to_string())?;
            while !*released {
                released = condition.wait(released).map_err(|_| "gate".to_string())?;
            }
        }
        self.inner.download(url, destination, kind)
    }

    fn make_executable(&mut self, path: &Path) -> Result<(), String> {
        self.inner.make_executable(path)
    }
}

#[test]
fn concurrent_preparations_have_one_owner_and_one_retryable_busy_result() {
    let root = clean_root("concurrency", 0);
    let _ = fs::remove_dir_all(&root);
    let package = PublishedPackage::parse(&test_lock_json("concurrent-package")).expect("lock");
    let gate = Arc::new((Mutex::new(false), Condvar::new()));
    let thread_root = root.clone();
    let thread_package = package.clone();
    let thread_gate = Arc::clone(&gate);
    let owner = thread::spawn(move || {
        let downloader = BlockingDownloader {
            inner: ControlledDownloader::default(),
            gate: thread_gate,
        };
        Acquisition::new(thread_root, downloader).prepare(&thread_package, HostPlatform::LinuxX8664)
    });

    while !root.join("install.lock").exists() {
        thread::yield_now();
    }
    let mut contender = Acquisition::new(&root, ControlledDownloader::default());
    assert_eq!(
        contender
            .prepare(&package, HostPlatform::LinuxX8664)
            .expect_err("contender should retry"),
        AcquisitionError::Busy
    );
    let (lock, condition) = &*gate;
    *lock.lock().expect("release lock") = true;
    condition.notify_one();
    assert!(owner.join().expect("owner thread").is_ok());
    assert!(!root.join("install.lock").exists());
    fs::remove_dir_all(root).expect("remove concurrency root");
}

#[test]
fn bounded_stale_lock_is_recovered_without_manual_deletion() {
    let root = clean_root("stale-lock", 0);
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("stale lock root");
    fs::write(
        root.join("install.lock"),
        b"schema_version=1\ncreated_unix_seconds=0\n",
    )
    .expect("stale lock");
    let package = PublishedPackage::parse(&test_lock_json("stale-lock-package")).expect("lock");
    let mut acquisition = Acquisition::new(&root, ControlledDownloader::default());

    let command = acquisition
        .prepare(&package, HostPlatform::LinuxX8664)
        .expect("stale lock recovery");

    assert!(command.is_file());
    assert!(!root.join("install.lock").exists());
    fs::remove_dir_all(root).expect("remove stale lock root");
}
