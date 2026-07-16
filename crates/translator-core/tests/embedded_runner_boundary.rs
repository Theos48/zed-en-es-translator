use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::time::Duration;

use translator_core::{EmbeddedProcessRunner, EmbeddedRunnerLimits, ProviderRequest};

fn script(name: &str, body: &str) -> (PathBuf, PathBuf) {
    let root = std::env::current_dir()
        .expect("current directory")
        .join("target/embedded-runner-tests")
        .join(format!("{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("create test root");
    let path = root.join("runner");
    fs::write(&path, format!("#!/bin/sh\nset -eu\n{body}\n")).expect("write runner");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).expect("set executable");
    (root, path)
}

#[test]
fn runner_should_return_one_ordered_batch() {
    let (root, executable) = script(
        "success",
        "cat >/dev/null; printf '%s' '{\"wire_version\":1,\"translations\":[\"Uno.\",\"Dos.\"]}'",
    );
    let runner = EmbeddedProcessRunner::from_verified_paths(
        executable,
        root.clone(),
        EmbeddedRunnerLimits::for_tests(Duration::from_secs(1)),
    )
    .expect("runner should construct");
    let request = ProviderRequest::new(
        vec!["One.".to_string(), "Two.".to_string()],
        translator_core::Language::English,
        translator_core::Language::Spanish,
        translator_core::Tone::TechnicalNeutral,
    )
    .expect("request should validate");

    let response = runner.run(&request).expect("runner should succeed");

    assert_eq!(response.translated_segments, vec!["Uno.", "Dos."]);
    fs::remove_dir_all(root).expect("remove runner root");
}

#[test]
fn runner_should_kill_and_reap_when_deadline_expires() {
    let (root, executable) = script("timeout", "cat >/dev/null; sleep 2");
    let runner = EmbeddedProcessRunner::from_verified_paths(
        executable,
        root.clone(),
        EmbeddedRunnerLimits::for_tests(Duration::from_millis(50)),
    )
    .expect("runner should construct");
    let request = ProviderRequest::new(
        vec!["One.".to_string()],
        translator_core::Language::English,
        translator_core::Language::Spanish,
        translator_core::Tone::TechnicalNeutral,
    )
    .expect("request should validate");

    let error = runner.run(&request).expect_err("deadline must fail");

    assert_eq!(error.code, translator_core::ErrorCode::ProviderTimeout);
    fs::remove_dir_all(root).expect("remove runner root");
}

#[test]
fn runner_should_reject_oversized_stderr_without_exposing_it() {
    let (root, executable) = script(
        "stderr",
        "cat >/dev/null; head -c 8192 /dev/zero >&2; exit 1",
    );
    let runner = EmbeddedProcessRunner::from_verified_paths(
        executable,
        root.clone(),
        EmbeddedRunnerLimits::for_tests(Duration::from_secs(1)),
    )
    .expect("runner should construct");
    let request = ProviderRequest::new(
        vec!["One.".to_string()],
        translator_core::Language::English,
        translator_core::Language::Spanish,
        translator_core::Tone::TechnicalNeutral,
    )
    .expect("request should validate");

    let error = runner.run(&request).expect_err("stderr cap must fail");

    assert_eq!(error.message, "Embedded provider process failed.");
    fs::remove_dir_all(root).expect("remove runner root");
}
