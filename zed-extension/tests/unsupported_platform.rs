mod common;

use std::path::Path;

use zed_en_es_translator_extension::acquisition::{
    Acquisition, AcquisitionError, DownloadKind, HostPlatform, PackageDownloader,
};
use zed_en_es_translator_extension::diagnostics::acquisition_message;
use zed_en_es_translator_extension::package::PublishedPackage;

use common::{clean_root, test_lock_json};

#[derive(Default)]
struct CountingDownloader {
    requests: usize,
    executable_changes: usize,
}

impl PackageDownloader for CountingDownloader {
    fn download(
        &mut self,
        _url: &str,
        _destination: &Path,
        _kind: DownloadKind,
    ) -> Result<(), String> {
        self.requests += 1;
        Err("unsupported platform must not download".to_string())
    }

    fn make_executable(&mut self, _path: &Path) -> Result<(), String> {
        self.executable_changes += 1;
        Err("unsupported platform must not mutate files".to_string())
    }
}

#[test]
fn every_unsupported_zed_platform_stops_before_network_or_storage() {
    let package = PublishedPackage::parse(&test_lock_json("unsupported-platform"))
        .expect("controlled package lock");
    let platforms = [
        HostPlatform::LinuxAarch64,
        HostPlatform::LinuxX86,
        HostPlatform::MacAarch64,
        HostPlatform::MacX86,
        HostPlatform::MacX8664,
        HostPlatform::WindowsAarch64,
        HostPlatform::WindowsX86,
        HostPlatform::WindowsX8664,
    ];

    for (index, platform) in platforms.into_iter().enumerate() {
        let root = clean_root("unsupported", index);
        let mut acquisition = Acquisition::new(&root, CountingDownloader::default());

        let error = acquisition
            .prepare(&package, platform)
            .expect_err("unsupported host must fail closed");

        assert_eq!(error, AcquisitionError::UnsupportedPlatform);
        assert_eq!(
            acquisition_message(error),
            "Automatic local translation currently supports Linux x86_64 only."
        );
        assert_eq!(acquisition.downloader().requests, 0);
        assert_eq!(acquisition.downloader().executable_changes, 0);
        assert!(!root.exists());
    }
}
