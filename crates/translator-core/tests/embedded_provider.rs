use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::time::Duration;

use serde_json::json;
use sha2::{Digest as _, Sha256};
use translator_core::{
    EmbeddedProcessProvider, EmbeddedProcessRunner, EmbeddedRunnerLimits, ErrorCode, Language,
    Provider, ProviderRequest, Tone,
};

fn runner(name: &str, body: &str) -> (PathBuf, EmbeddedProcessRunner) {
    let root = std::env::current_dir()
        .expect("current directory")
        .join("target/embedded-provider-tests")
        .join(format!("{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("create runner root");
    let executable = root.join("translator-embedded-runtime");
    fs::write(&executable, format!("#!/bin/sh\nset -eu\n{body}\n")).expect("write runner");
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o700))
        .expect("make runner executable");
    let runner = EmbeddedProcessRunner::from_verified_paths(
        executable,
        root.clone(),
        EmbeddedRunnerLimits::for_tests(Duration::from_millis(200)),
    )
    .expect("verified test runner");
    (root, runner)
}

fn adjacent_package(name: &str) -> (PathBuf, PathBuf) {
    let package_id = format!("test-package-{}-{name}", std::process::id());
    let parent = std::env::current_dir()
        .expect("current directory")
        .join("target/embedded-package-tests")
        .join(format!("root-{package_id}"));
    let package = parent.join(&package_id);
    let _ = fs::remove_dir_all(&package);
    fs::create_dir_all(package.join("bin")).expect("create package bin");
    fs::create_dir_all(package.join("models")).expect("create package models");

    let files = [
        (
            "language_server",
            "bin/translator-lsp",
            b"#!/bin/sh\nexit 0\n".as_slice(),
            true,
        ),
        (
            "native_runner",
            "bin/translator-embedded-runtime",
            b"#!/bin/sh\nwhile IFS= read -r _; do :; done\nprintf '%s' '{\"wire_version\":1,\"translations\":[\"Uno.\",\"Dos.\"]}'\n"
                .as_slice(),
            true,
        ),
        (
            "model",
            "models/model.enes.intgemm.alphas.bin",
            b"controlled-model".as_slice(),
            false,
        ),
        (
            "vocabulary",
            "models/vocab.enes.spm",
            b"controlled-vocabulary".as_slice(),
            false,
        ),
        (
            "lexical_shortlist",
            "models/lex.50.50.enes.s2t.bin",
            b"controlled-lexical".as_slice(),
            false,
        ),
    ];
    let artifacts = files
        .iter()
        .map(|(role, path, contents, executable)| {
            let absolute = package.join(path);
            fs::write(&absolute, contents).expect("write package artifact");
            let mode = if *executable { 0o700 } else { 0o600 };
            fs::set_permissions(&absolute, fs::Permissions::from_mode(mode))
                .expect("set package artifact permissions");
            json!({
                "role": role,
                "path": path,
                "installed_size": contents.len(),
                "installed_sha256": hex_sha256(contents),
                "executable": executable,
            })
        })
        .collect::<Vec<_>>();
    let manifest = json!({
        "schema_version": 1,
        "package_id": package_id,
        "package_version": "0.1.0",
        "platform": "linux-x86_64",
        "source_language": "en",
        "target_language": "es",
        "wire_version": 1,
        "state": "verified",
        "artifacts": artifacts,
    });
    let manifest_path = package.join("installed.json");
    fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&manifest).expect("serialize manifest"),
    )
    .expect("write manifest");
    fs::set_permissions(&manifest_path, fs::Permissions::from_mode(0o600))
        .expect("set manifest permissions");
    (parent, package)
}

fn hex_sha256(contents: &[u8]) -> String {
    Sha256::digest(contents)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn request() -> ProviderRequest {
    ProviderRequest::new(
        vec!["One.".to_string(), "Two.".to_string()],
        Language::English,
        Language::Spanish,
        Tone::TechnicalNeutral,
    )
    .expect("provider request")
}

#[test]
fn embedded_provider_returns_one_ordered_batch() {
    let (root, runner) = runner(
        "ordered",
        "while IFS= read -r _; do :; done; printf '%s' '{\"wire_version\":1,\"translations\":[\"Uno.\",\"Dos.\"]}'",
    );
    let provider = EmbeddedProcessProvider::from_verified_runner(runner);

    let response = provider.translate(&request()).expect("offline translation");

    assert_eq!(response.translated_segments, ["Uno.", "Dos."]);
    fs::remove_dir_all(root).expect("remove runner root");
}

#[test]
fn embedded_provider_does_not_fallback_after_invalid_response() {
    let (root, runner) = runner(
        "invalid",
        "while IFS= read -r _; do :; done; printf '%s' '{\"wire_version\":1,\"translations\":[\"mock-looking\"]}'",
    );
    let provider = EmbeddedProcessProvider::from_verified_runner(runner);

    let failure = provider
        .translate(&request())
        .expect_err("cardinality mismatch must fail");

    assert_eq!(failure.code, ErrorCode::ProviderFailed);
    fs::remove_dir_all(root).expect("remove runner root");
}

#[test]
fn embedded_provider_preserves_the_existing_total_deadline() {
    let (root, runner) = runner("timeout", "while IFS= read -r _; do :; done; sleep 1");
    let provider = EmbeddedProcessProvider::from_verified_runner(runner);

    let failure = provider
        .translate(&request())
        .expect_err("timeout must fail");

    assert_eq!(failure.code, ErrorCode::ProviderTimeout);
    fs::remove_dir_all(root).expect("remove runner root");
}

#[test]
fn adjacent_verified_package_resolves_the_fixed_runner_and_models() {
    let (parent, package) = adjacent_package("valid");
    let provider = EmbeddedProcessProvider::from_package_root(&package)
        .expect("adjacent package should verify");

    let response = provider
        .translate(&request())
        .expect("verified package should translate");

    assert_eq!(response.translated_segments, ["Uno.", "Dos."]);
    fs::remove_dir_all(parent).expect("remove package test root");
}

#[test]
fn adjacent_package_rejects_tampering_without_revealing_a_path() {
    let (parent, package) = adjacent_package("tampered");
    fs::write(
        package.join("models/model.enes.intgemm.alphas.bin"),
        b"tampered-model",
    )
    .expect("tamper model");

    let failure = EmbeddedProcessProvider::from_package_root(&package)
        .expect_err("tampered package must fail closed");

    assert_eq!(failure.code, ErrorCode::ProviderNotConfigured);
    assert!(!failure.message.contains("target"));
    assert!(!failure.message.contains("models"));
    fs::remove_dir_all(parent).expect("remove package test root");
}
