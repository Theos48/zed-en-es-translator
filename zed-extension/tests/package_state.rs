mod common;

use std::fs;

use zed_en_es_translator_extension::acquisition::{Acquisition, HostPlatform};
use zed_en_es_translator_extension::package::PublishedPackage;

use common::{clean_root, test_lock_json, ControlledDownloader};

#[test]
fn successful_updates_retain_only_active_and_previous_generations() {
    let root = clean_root("state-transition", 0);
    let _ = fs::remove_dir_all(&root);
    let first = PublishedPackage::parse(&test_lock_json("package-v1")).expect("v1 lock");
    let second = PublishedPackage::parse(&test_lock_json("package-v2")).expect("v2 lock");
    let third = PublishedPackage::parse(&test_lock_json("package-v3")).expect("v3 lock");

    let mut acquisition = Acquisition::new(&root, ControlledDownloader::default());
    acquisition
        .prepare(&first, HostPlatform::LinuxX8664)
        .expect("prepare v1");
    acquisition
        .prepare(&second, HostPlatform::LinuxX8664)
        .expect("prepare v2");
    acquisition
        .prepare(&third, HostPlatform::LinuxX8664)
        .expect("prepare v3");

    let state: serde_json::Value =
        serde_json::from_slice(&fs::read(root.join("state.json")).expect("state file"))
            .expect("state JSON");
    assert_eq!(state["active"], "package-v3");
    assert_eq!(state["previous"], "package-v2");
    assert_eq!(state["generation"], 3);
    assert!(!root.join("packages/package-v1").exists());
    assert!(root.join("packages/package-v2").is_dir());
    assert!(root.join("packages/package-v3").is_dir());
    assert!(!root.join("state.next.json").exists());
    fs::remove_dir_all(root).expect("remove state root");
}
