use zed_en_es_translator_extension::package::PublishedPackage;
use zed_extension_api::serde_json::{self, json, Value};

fn valid_lock() -> Value {
    let model_lock: Value =
        serde_json::from_str(include_str!("../../ops/marketplace/model.lock.json"))
            .expect("model lock");
    json!({
        "schema_version": 1,
        "package_id": "en-es-translator-0.1.0-linux-x86_64",
        "package_version": "0.1.0",
        "platform": "linux-x86_64",
        "source_language": "en",
        "target_language": "es",
        "server_archive": {
            "url": "https://github.com/Theos48/zed-en-es-translator/releases/download/v0.1.0/en-es-translator-0.1.0-linux-x86_64.tar.gz",
            "archive_type": "gzip_tar",
            "files": [
                server_file("language_server", "bin/translator-lsp", true, '1'),
                server_file("native_runner", "bin/translator-embedded-runtime", true, '2'),
                server_file("notice", "LICENSES/THIRD_PARTY_NOTICES.md", false, '3'),
                server_file("license", "LICENSES/MPL-2.0.txt", false, '4'),
                server_file("source_instructions", "LICENSES/SOURCE.md", false, '5')
            ]
        },
        "model_resources": model_lock["model_resources"].clone(),
        "budgets": model_lock["budgets"].clone(),
        "license_bundle": {
            "extension_spdx": "MIT",
            "required_paths": [
                "LICENSES/THIRD_PARTY_NOTICES.md",
                "LICENSES/MPL-2.0.txt",
                "LICENSES/SOURCE.md"
            ]
        }
    })
}

fn server_file(role: &str, path: &str, executable: bool, digit: char) -> Value {
    json!({
        "role": role,
        "path": path,
        "installed_size": 1024,
        "installed_sha256": digit.to_string().repeat(64),
        "executable": executable,
        "spdx_conclusion": if role == "language_server" { "MIT" } else { "MPL-2.0" },
        "source_url": "https://github.com/Theos48/zed-en-es-translator"
    })
}

fn parse(value: &Value) -> Result<PublishedPackage, String> {
    PublishedPackage::parse(&serde_json::to_string(value).expect("serialize lock"))
        .map_err(|error| error.to_string())
}

#[test]
fn strict_published_lock_accepts_the_fixed_complete_profile() {
    let package = parse(&valid_lock()).expect("valid published package");

    assert_eq!(package.package_id(), "en-es-translator-0.1.0-linux-x86_64");
    assert_eq!(package.model_resources().len(), 3);
    assert_eq!(package.server_files().len(), 5);
}

#[test]
fn strict_published_lock_rejects_unknown_fields() {
    let mut lock = valid_lock();
    lock["unexpected"] = json!(true);

    assert!(parse(&lock).is_err());
}

#[test]
fn published_lock_rejects_unsafe_paths_and_duplicate_roles() {
    let mut unsafe_path = valid_lock();
    unsafe_path["server_archive"]["files"][0]["path"] = json!("../translator-lsp");
    assert!(parse(&unsafe_path).is_err());

    let mut duplicate = valid_lock();
    duplicate["model_resources"][1]["role"] = json!("model");
    assert!(parse(&duplicate).is_err());
}

#[test]
fn published_lock_rejects_budget_drift_and_zero_hashes() {
    let mut budget = valid_lock();
    budget["budgets"]["maximum_transfer_bytes"] = json!(67_108_865_u64);
    assert!(parse(&budget).is_err());

    let mut hash = valid_lock();
    hash["server_archive"]["files"][0]["installed_sha256"] = json!("0".repeat(64));
    assert!(parse(&hash).is_err());
}
