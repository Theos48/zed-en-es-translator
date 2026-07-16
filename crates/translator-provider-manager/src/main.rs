use std::fs::{self, File};
use std::io::Read;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;
use std::process::ExitCode;

use serde_json::Value;
use sha2::{Digest as _, Sha256};
use translator_provider_manager::acquisition::AcquisitionPolicy;
use translator_provider_manager::cleanup::clean;
use translator_provider_manager::cli::Command;
use translator_provider_manager::disclosure::Disclosure;
use translator_provider_manager::error::ManagerError;
use translator_provider_manager::lifecycle::{ControlledArtifact, Lifecycle};
use translator_provider_manager::manifest::ProviderManifest;
use translator_provider_manager::status::{status as lifecycle_status, verify};
use translator_provider_manager::storage::StorageRoot;

const MANIFEST_PATH: &str = "ops/providers/embedded/provider.lock.json";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("provider_status={}", error.code());
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), ManagerError> {
    let arguments = std::env::args_os()
        .skip(1)
        .map(|argument| {
            argument
                .into_string()
                .map_err(|_| ManagerError::StateInvalid)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let command = Command::parse(&arguments)?;
    match command {
        Command::Disclose => disclose(),
        Command::Status => status(),
        Command::Prepare { consent } | Command::Update { consent } => prepare(&consent),
        Command::Verify => verify_state(),
        Command::Rollback => {
            let root = StorageRoot::fixed_path()?;
            Lifecycle::new(root).rollback()
        }
        Command::Clean { confirmation } => {
            let root = StorageRoot::fixed_path()?;
            clean(&root, &confirmation)
        }
        Command::BuildRunner => Err(ManagerError::ApprovalRequired),
    }
}

fn disclose() -> Result<(), ManagerError> {
    let input = fs::read(MANIFEST_PATH).map_err(|_| ManagerError::ManifestInvalid)?;
    let text = std::str::from_utf8(&input).map_err(|_| ManagerError::ManifestInvalid)?;
    if let Ok(manifest) = ProviderManifest::from_json(text) {
        print!("{}", Disclosure::from_manifest(&manifest)?.render());
        return Ok(());
    }
    let raw: Value = serde_json::from_slice(&input).map_err(|_| ManagerError::ManifestInvalid)?;
    let review = raw
        .get("review_status")
        .and_then(Value::as_str)
        .unwrap_or("invalid");
    let publication = raw
        .get("publication_status")
        .and_then(Value::as_str)
        .unwrap_or("invalid");
    println!("profile=bergamot-en-es-linux-x86_64-v1");
    println!("language=en-es");
    println!("scope=user_xdg_data");
    println!("normal_translation_network=none");
    println!("review_status={review}");
    println!("publication={publication}");
    println!("consent_available=false");
    println!("review_lock_digest={}", sha256(&input));
    Ok(())
}

fn status() -> Result<(), ManagerError> {
    let root = StorageRoot::fixed_path()?;
    print!("{}", lifecycle_status(&root)?.render());
    Ok(())
}

fn prepare(consent: &str) -> Result<(), ManagerError> {
    let input = fs::read_to_string(MANIFEST_PATH).map_err(|_| ManagerError::ManifestInvalid)?;
    let manifest = ProviderManifest::from_json(&input).map_err(|error| {
        let raw: Option<Value> = serde_json::from_str(&input).ok();
        if raw.as_ref().is_some_and(|value| {
            value.get("review_status").and_then(Value::as_str) != Some("approved")
                || value.get("local_approval").is_none_or(Value::is_null)
        }) {
            ManagerError::ApprovalRequired
        } else {
            error
        }
    })?;
    if consent != manifest.artifact_set_digest() {
        return Err(ManagerError::ConsentRequired);
    }
    let runner = read_runner(
        Path::new("target/embedded-native-release/translator-embedded-runtime"),
        manifest.runner().size(),
    )?;
    let sources = manifest
        .artifacts()
        .iter()
        .map(|artifact| {
            let expected_size = usize::try_from(artifact.compressed_size())
                .map_err(|_| ManagerError::ManifestInvalid)?;
            let policy = AcquisitionPolicy::new(
                artifact.attachment_url(),
                expected_size,
                artifact.compressed_sha256(),
            )?;
            Ok(ControlledArtifact {
                role: artifact.role().to_string(),
                compressed: policy.acquire_https()?,
            })
        })
        .collect::<Result<Vec<_>, ManagerError>>()?;
    let root = StorageRoot::fixed_path()?;
    Lifecycle::new(root).prepare_with_offline_smoke(&manifest, consent, &runner, &sources)?;
    println!("provider_status=ready");
    Ok(())
}

fn read_runner(path: &Path, expected_size: u64) -> Result<Vec<u8>, ManagerError> {
    let metadata = fs::symlink_metadata(path).map_err(|_| ManagerError::IntegrityFailed)?;
    // SAFETY: `geteuid` has no arguments and no safety preconditions.
    let effective_uid = unsafe { libc::geteuid() };
    if metadata.file_type().is_symlink()
        || !metadata.is_file()
        || metadata.uid() != effective_uid
        || metadata.nlink() != 1
        || metadata.permissions().mode() & 0o022 != 0
        || metadata.len() != expected_size
    {
        return Err(ManagerError::IntegrityFailed);
    }
    let capacity = usize::try_from(expected_size).map_err(|_| ManagerError::IntegrityFailed)?;
    let mut runner = Vec::with_capacity(capacity);
    File::open(path)
        .and_then(|file| file.take(expected_size + 1).read_to_end(&mut runner))
        .map_err(|_| ManagerError::IntegrityFailed)?;
    if runner.len() != capacity {
        return Err(ManagerError::IntegrityFailed);
    }
    Ok(runner)
}

fn verify_state() -> Result<(), ManagerError> {
    let root = StorageRoot::fixed_path()?;
    verify(&root)?;
    println!("provider_status=verified");
    Ok(())
}

fn sha256(content: &[u8]) -> String {
    Sha256::digest(content)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
