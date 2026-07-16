mod common;

use std::fs;
use std::time::Duration;

use zed_en_es_translator_extension::acquisition::{Acquisition, HostPlatform};
use zed_en_es_translator_extension::package::PublishedPackage;

use common::{clean_root, test_lock_json, ControlledDownloader};

#[test]
fn twenty_clean_controlled_preparations_meet_the_five_minute_threshold() {
    let mut preparations_under_five_minutes = 0;
    for iteration in 0..20 {
        let package_id = format!("controlled-package-{iteration}");
        let package =
            PublishedPackage::parse(&test_lock_json(&package_id)).expect("controlled lock");
        let root = clean_root("happy", iteration);
        let _ = fs::remove_dir_all(&root);
        let downloader = ControlledDownloader::default();
        let mut acquisition = Acquisition::new(&root, downloader);

        let command = acquisition
            .prepare(&package, HostPlatform::LinuxX8664)
            .expect("clean preparation");

        assert_eq!(
            command,
            root.join("packages")
                .join(&package_id)
                .join("bin/translator-lsp")
        );
        assert!(command.is_file());
        assert_eq!(acquisition.downloader().requests.len(), 4);
        if acquisition.downloader().simulated_duration_at_10_mbps() < Duration::from_secs(5 * 60) {
            preparations_under_five_minutes += 1;
        }
        fs::remove_dir_all(root).expect("remove controlled root");
    }

    assert!(preparations_under_five_minutes >= 19);
}

#[test]
fn ready_package_is_reused_without_any_network_request() {
    let package =
        PublishedPackage::parse(&test_lock_json("controlled-reuse")).expect("controlled lock");
    let root = clean_root("reuse", 0);
    let _ = fs::remove_dir_all(&root);
    let mut first = Acquisition::new(&root, ControlledDownloader::default());
    let command = first
        .prepare(&package, HostPlatform::LinuxX8664)
        .expect("initial preparation");
    drop(first);

    let mut second = Acquisition::new(&root, ControlledDownloader::default());
    let reused = second
        .prepare(&package, HostPlatform::LinuxX8664)
        .expect("offline reuse");

    assert_eq!(reused, command);
    assert!(second.downloader().requests.is_empty());
    fs::remove_dir_all(root).expect("remove controlled root");
}
