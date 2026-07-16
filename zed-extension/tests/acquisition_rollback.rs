mod common;

use std::fs;
use std::path::Path;

use zed_en_es_translator_extension::acquisition::{
    Acquisition, DownloadKind, HostPlatform, PackageDownloader,
};
use zed_en_es_translator_extension::package::PublishedPackage;

use common::{clean_root, test_lock_json, ControlledDownloader};

struct CorruptUpdateDownloader(ControlledDownloader);

impl PackageDownloader for CorruptUpdateDownloader {
    fn download(
        &mut self,
        url: &str,
        destination: &Path,
        kind: DownloadKind,
    ) -> Result<(), String> {
        self.0.download(url, destination, kind)?;
        if kind == DownloadKind::Uncompressed {
            fs::write(destination, b"invalid update").map_err(|_| "corrupt update".to_string())?;
        }
        Ok(())
    }

    fn make_executable(&mut self, path: &Path) -> Result<(), String> {
        self.0.make_executable(path)
    }
}

#[test]
fn invalid_update_keeps_and_returns_the_last_known_good_server() {
    let root = clean_root("rollback", 0);
    let _ = fs::remove_dir_all(&root);
    let stable = PublishedPackage::parse(&test_lock_json("stable-package")).expect("stable lock");
    let update = PublishedPackage::parse(&test_lock_json("invalid-update")).expect("update lock");
    let mut first = Acquisition::new(&root, ControlledDownloader::default());
    let stable_command = first
        .prepare(&stable, HostPlatform::LinuxX8664)
        .expect("stable package");
    drop(first);

    let mut update_acquisition = Acquisition::new(
        &root,
        CorruptUpdateDownloader(ControlledDownloader::default()),
    );
    let selected = update_acquisition
        .prepare(&update, HostPlatform::LinuxX8664)
        .expect("last-known-good fallback");

    assert_eq!(selected, stable_command);
    assert!(stable_command.is_file());
    assert!(!root.join("packages/invalid-update").exists());
    let state: serde_json::Value =
        serde_json::from_slice(&fs::read(root.join("state.json")).expect("state"))
            .expect("state JSON");
    assert_eq!(state["active"], "stable-package");
    fs::remove_dir_all(root).expect("remove rollback root");
}
