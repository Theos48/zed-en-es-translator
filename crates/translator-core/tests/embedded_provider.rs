use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::time::Duration;

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

fn request(remote_confirmed: bool) -> ProviderRequest {
    ProviderRequest::with_remote_confirmation(
        vec!["One.".to_string(), "Two.".to_string()],
        Language::English,
        Language::Spanish,
        Tone::TechnicalNeutral,
        remote_confirmed,
    )
    .expect("provider request")
}

#[test]
fn embedded_provider_returns_one_ordered_batch_without_remote_confirmation() {
    let (root, runner) = runner(
        "ordered",
        "while IFS= read -r _; do :; done; printf '%s' '{\"wire_version\":1,\"translations\":[\"Uno.\",\"Dos.\"]}'",
    );
    let provider = EmbeddedProcessProvider::from_verified_runner(runner);

    let response = provider
        .translate(&request(false))
        .expect("offline translation");

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
        .translate(&request(true))
        .expect_err("cardinality mismatch must fail");

    assert_eq!(failure.code, ErrorCode::ProviderFailed);
    fs::remove_dir_all(root).expect("remove runner root");
}

#[test]
fn embedded_provider_preserves_the_existing_total_deadline() {
    let (root, runner) = runner("timeout", "while IFS= read -r _; do :; done; sleep 1");
    let provider = EmbeddedProcessProvider::from_verified_runner(runner);

    let failure = provider
        .translate(&request(false))
        .expect_err("timeout must fail");

    assert_eq!(failure.code, ErrorCode::ProviderTimeout);
    fs::remove_dir_all(root).expect("remove runner root");
}
