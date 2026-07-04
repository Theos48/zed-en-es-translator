use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use zed_en_es_translator_extension::diagnostics::DiagnosticCode;
use zed_en_es_translator_extension::launch::{build_launch_profile, CONTEXT_SERVER_ID};
use zed_en_es_translator_extension::settings::{
    CommandSettingsInput, LaunchSettings, FORBIDDEN_SETTING_NAMES,
};
use zed_extension_api::serde_json::json;

#[test]
fn launch_settings_accepts_allowed_binary_path() {
    let settings = LaunchSettings::from_json_value(Some(&json!({
        "binary_path": "/tmp/translator-mcp"
    })))
    .expect("binary_path should be accepted");

    assert_eq!(settings.binary_path(), Some("/tmp/translator-mcp"));
}

#[test]
fn launch_settings_rejects_empty_binary_path_with_specific_diagnostic() {
    let error = LaunchSettings::from_json_value(Some(&json!({
        "binary_path": "  "
    })))
    .expect_err("empty binary_path should be rejected");

    assert_eq!(error.code, DiagnosticCode::BinaryPathNotConfigured);
    assert!(!error.to_user_message().contains("unsupported"));
}

#[test]
fn launch_settings_rejects_provider_settings() {
    for key in FORBIDDEN_SETTING_NAMES {
        let error = LaunchSettings::from_json_value(Some(&json!({
            (*key): "unsafe"
        })))
        .expect_err("forbidden setting should be rejected");

        assert_eq!(error.code, DiagnosticCode::UnsafeLaunchConfiguration);
    }
}

#[test]
fn launch_settings_rejects_extra_args_and_arbitrary_env() {
    let error = LaunchSettings::from_parts(
        None,
        CommandSettingsInput::new(
            Some("/tmp/translator-mcp".to_string()),
            vec!["--unsafe".to_string()],
            vec![("SECRET_TOKEN".to_string(), "fake".to_string())],
        ),
    )
    .expect_err("extra args should be rejected");

    assert_eq!(error.code, DiagnosticCode::UnsafeLaunchConfiguration);
}

#[test]
fn launch_settings_accepts_only_allowlisted_rust_log_env() {
    let settings = LaunchSettings::from_parts(
        None,
        CommandSettingsInput::new(
            Some("/tmp/translator-mcp".to_string()),
            Vec::new(),
            vec![("RUST_LOG".to_string(), "warn".to_string())],
        ),
    )
    .expect("RUST_LOG=warn should be accepted");

    assert_eq!(settings.rust_log(), Some("warn"));
}

#[test]
fn build_launch_profile_returns_direct_translator_command() {
    let artifact = executable_artifact("direct-command");
    let settings = LaunchSettings::from_json_value(Some(&json!({
        "binary_path": artifact.display().to_string()
    })))
    .expect("binary_path should parse");

    let profile =
        build_launch_profile(CONTEXT_SERVER_ID, &settings).expect("launch profile should build");

    assert_eq!(profile.command, artifact.display().to_string());
    assert!(profile.args.is_empty());
    assert!(profile.env.is_empty());
}

#[test]
fn unsupported_context_server_id_returns_stable_error() {
    let settings = LaunchSettings::from_json_value(Some(&json!({
        "binary_path": "/tmp/translator-mcp"
    })))
    .expect("binary_path should parse");

    let error = build_launch_profile("other-server", &settings)
        .expect_err("unsupported context server should fail");

    assert_eq!(error.code, DiagnosticCode::UnsupportedContextServer);
}

#[test]
fn path_with_spaces_remains_one_command_value() {
    let artifact = executable_artifact("path with spaces");
    let settings = LaunchSettings::from_json_value(Some(&json!({
        "binary_path": artifact.display().to_string()
    })))
    .expect("binary_path should parse");

    let profile =
        build_launch_profile(CONTEXT_SERVER_ID, &settings).expect("launch profile should build");

    assert_eq!(profile.command, artifact.display().to_string());
    assert_eq!(profile.args, Vec::<String>::new());
}

#[test]
fn remote_provider_settings_are_rejected() {
    let error = LaunchSettings::from_json_value(Some(&json!({
        "remote": true,
        "provider": "network"
    })))
    .expect_err("remote provider settings should be rejected");

    assert_eq!(error.code, DiagnosticCode::UnsafeLaunchConfiguration);
}

#[test]
fn repeated_startup_failure_revalidates_without_state() {
    let settings = LaunchSettings::from_json_value(Some(&json!({
        "binary_path": "/tmp/missing/translator-mcp"
    })))
    .expect("binary_path should parse");

    let first = build_launch_profile(CONTEXT_SERVER_ID, &settings)
        .expect_err("missing artifact should fail");
    let second = build_launch_profile(CONTEXT_SERVER_ID, &settings)
        .expect_err("missing artifact should fail again");

    assert_eq!(first.to_user_message(), second.to_user_message());
}

fn executable_artifact(case: &str) -> PathBuf {
    let dir = unique_temp_dir(case);
    fs::create_dir_all(&dir).expect("temp directory should be created");
    let path = dir.join("translator-mcp");
    fs::write(&path, "#!/bin/sh\nexit 0\n").expect("artifact should be written");
    make_executable(&path);
    path
}

fn unique_temp_dir(case: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "zed-extension-{case}-{}-{nanos}",
        std::process::id()
    ))
}

#[cfg(unix)]
fn make_executable(path: &PathBuf) {
    let mut permissions = fs::metadata(path)
        .expect("artifact metadata should be readable")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).expect("artifact should be executable");
}

#[cfg(not(unix))]
fn make_executable(_path: &PathBuf) {}
