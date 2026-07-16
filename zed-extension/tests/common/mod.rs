#![allow(dead_code)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::time::Duration;

use serde_json::{json, Value};
use sha2::{Digest as _, Sha256};
use zed_en_es_translator_extension::acquisition::{DownloadKind, PackageDownloader};

pub const MODEL_BYTES: &[u8] = b"controlled-model";
pub const VOCABULARY_BYTES: &[u8] = b"controlled-vocabulary";
pub const LEXICAL_BYTES: &[u8] = b"controlled-lexical";
pub const MODEL_ZSTD: &[u8] = &[
    0x28, 0xb5, 0x2f, 0xfd, 0x04, 0x58, 0x81, 0x00, 0x00, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x6c, 0x65, 0x64, 0x2d, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x30, 0x15, 0x21, 0xb8,
];
pub const VOCABULARY_ZSTD: &[u8] = &[
    0x28, 0xb5, 0x2f, 0xfd, 0x04, 0x58, 0xa9, 0x00, 0x00, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x6c, 0x65, 0x64, 0x2d, 0x76, 0x6f, 0x63, 0x61, 0x62, 0x75, 0x6c, 0x61, 0x72, 0x79, 0x90, 0x49,
    0x15, 0x87,
];
pub const LEXICAL_ZSTD: &[u8] = &[
    0x28, 0xb5, 0x2f, 0xfd, 0x04, 0x58, 0x91, 0x00, 0x00, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x6c, 0x65, 0x64, 0x2d, 0x6c, 0x65, 0x78, 0x69, 0x63, 0x61, 0x6c, 0xdf, 0x88, 0x54, 0xdd,
];

const SERVER_FILES: &[(&str, &str, &[u8], bool)] = &[
    (
        "language_server",
        "bin/translator-lsp",
        b"controlled-language-server",
        true,
    ),
    (
        "native_runner",
        "bin/translator-embedded-runtime",
        b"controlled-native-runner",
        true,
    ),
    (
        "notice",
        "LICENSES/THIRD_PARTY_NOTICES.md",
        b"controlled notices",
        false,
    ),
    (
        "license",
        "LICENSES/MPL-2.0.txt",
        b"controlled license",
        false,
    ),
    (
        "source_instructions",
        "LICENSES/SOURCE.md",
        b"controlled source instructions",
        false,
    ),
];

pub fn test_lock_json(package_id: &str) -> String {
    let server_files = SERVER_FILES
        .iter()
        .map(|(role, path, contents, executable)| {
            json!({
                "role": role,
                "path": path,
                "installed_size": contents.len(),
                "installed_sha256": sha256(contents),
                "executable": executable,
                "spdx_conclusion": if *role == "language_server" { "MIT" } else { "MPL-2.0" },
                "source_url": "https://github.com/Theos48/zed-en-es-translator"
            })
        })
        .collect::<Vec<_>>();
    let resources = [
        model_resource(
            "model",
            "00000000-0000-0000-0000-000000000001",
            "model.zst",
            MODEL_ZSTD,
            "model.enes.intgemm.alphas.bin",
            MODEL_BYTES,
        ),
        model_resource(
            "vocabulary",
            "00000000-0000-0000-0000-000000000002",
            "vocabulary.zst",
            VOCABULARY_ZSTD,
            "vocab.enes.spm",
            VOCABULARY_BYTES,
        ),
        model_resource(
            "lexical_shortlist",
            "00000000-0000-0000-0000-000000000003",
            "lexical.zst",
            LEXICAL_ZSTD,
            "lex.50.50.enes.s2t.bin",
            LEXICAL_BYTES,
        ),
    ];
    serde_json::to_string_pretty(&json!({
        "schema_version": 1,
        "package_id": package_id,
        "package_version": "0.1.0",
        "platform": "linux-x86_64",
        "source_language": "en",
        "target_language": "es",
        "server_archive": {
            "url": "https://github.com/Theos48/zed-en-es-translator/releases/download/v0.1.0/package.tar.gz",
            "archive_type": "gzip_tar",
            "files": server_files
        },
        "model_resources": resources,
        "budgets": {
            "maximum_transfer_bytes": 67108864,
            "maximum_active_installed_bytes": 134217728,
            "maximum_lifecycle_bytes": 402653184,
            "required_free_bytes": 536870912,
            "peak_rss_bytes": 1073741824,
            "inference_threads": 4,
            "provider_deadline_ms": 15000
        },
        "license_bundle": {
            "extension_spdx": "MIT",
            "required_paths": [
                "LICENSES/THIRD_PARTY_NOTICES.md",
                "LICENSES/MPL-2.0.txt",
                "LICENSES/SOURCE.md"
            ]
        }
    }))
    .expect("serialize test lock")
}

fn model_resource(
    role: &str,
    record_id: &str,
    compressed_name: &str,
    compressed: &[u8],
    installed_name: &str,
    installed: &[u8],
) -> Value {
    json!({
        "role": role,
        "record_id": record_id,
        "url": format!("https://models.example.invalid/{compressed_name}"),
        "compressed_name": compressed_name,
        "compressed_size": compressed.len(),
        "compressed_sha256": sha256(compressed),
        "installed_name": installed_name,
        "installed_size": installed.len(),
        "installed_sha256": sha256(installed),
        "spdx_conclusion": "MPL-2.0",
        "license_url": "https://github.com/mozilla/firefox-translations-models/blob/example/LICENSE"
    })
}

pub fn sha256(contents: &[u8]) -> String {
    Sha256::digest(contents)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[derive(Default)]
pub struct ControlledDownloader {
    pub requests: Vec<(String, DownloadKind)>,
    pub transferred_bytes: u64,
}

impl ControlledDownloader {
    pub fn simulated_duration_at_10_mbps(&self) -> Duration {
        let transfer_seconds = (self.transferred_bytes as f64 * 8.0) / 10_000_000.0;
        Duration::from_secs_f64(transfer_seconds)
            + Duration::from_millis(self.requests.len() as u64 * 25)
    }
}

impl PackageDownloader for ControlledDownloader {
    fn download(
        &mut self,
        url: &str,
        destination: &Path,
        kind: DownloadKind,
    ) -> Result<(), String> {
        self.requests.push((url.to_string(), kind));
        match kind {
            DownloadKind::GzipTar => {
                for (_, path, contents, executable) in SERVER_FILES {
                    let output = destination.join(path);
                    fs::create_dir_all(output.parent().expect("server file parent"))
                        .map_err(|_| "fixture directory".to_string())?;
                    fs::write(&output, contents).map_err(|_| "fixture file".to_string())?;
                    let mode = if *executable { 0o700 } else { 0o600 };
                    fs::set_permissions(&output, fs::Permissions::from_mode(mode))
                        .map_err(|_| "fixture permissions".to_string())?;
                    self.transferred_bytes += contents.len() as u64;
                }
            }
            DownloadKind::Uncompressed => {
                let contents = if url.ends_with("model.zst") {
                    MODEL_ZSTD
                } else if url.ends_with("vocabulary.zst") {
                    VOCABULARY_ZSTD
                } else if url.ends_with("lexical.zst") {
                    LEXICAL_ZSTD
                } else {
                    return Err("unknown controlled URL".to_string());
                };
                if let Some(parent) = destination.parent() {
                    fs::create_dir_all(parent).map_err(|_| "fixture directory".to_string())?;
                }
                fs::write(destination, contents).map_err(|_| "fixture file".to_string())?;
                self.transferred_bytes += contents.len() as u64;
            }
        }
        Ok(())
    }

    fn make_executable(&mut self, path: &Path) -> Result<(), String> {
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))
            .map_err(|_| "fixture executable".to_string())
    }
}

pub fn clean_root(case: &str, iteration: usize) -> std::path::PathBuf {
    std::env::current_dir()
        .expect("current directory")
        .join("target/marketplace-acquisition-tests")
        .join(format!("{}-{case}-{iteration}", std::process::id()))
}
