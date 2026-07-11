use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use zed_en_es_translator_extension::diagnostics::DiagnosticCode;
use zed_en_es_translator_extension::launch::{build_launch_profile, CONTEXT_SERVER_ID};
use zed_en_es_translator_extension::settings::LaunchSettings;
use zed_extension_api::serde_json::json;

#[test]
fn provider_settings_map_to_controlled_launch_environment() {
    let artifact = executable_artifact("provider-env");
    let settings = LaunchSettings::from_json_value(Some(&json!({
        "binary_path": artifact.display().to_string(),
        "provider": {
            "mode": "libretranslate",
            "url": "http://127.0.0.1:5000",
            "api_key_env": "TRANSLATOR_TEST_API_KEY",
            "allow_remote": false
        }
    })))
    .expect("provider settings should parse");

    let profile =
        build_launch_profile(CONTEXT_SERVER_ID, &settings).expect("launch profile should build");

    assert_eq!(
        profile.env,
        vec![
            (
                "TRANSLATOR_PROVIDER".to_string(),
                "libretranslate".to_string()
            ),
            (
                "TRANSLATOR_PROVIDER_URL".to_string(),
                "http://127.0.0.1:5000".to_string()
            ),
            (
                "TRANSLATOR_PROVIDER_API_KEY_ENV".to_string(),
                "TRANSLATOR_TEST_API_KEY".to_string()
            ),
            (
                "TRANSLATOR_ALLOW_REMOTE_PROVIDER".to_string(),
                "false".to_string()
            )
        ]
    );
}

#[test]
fn provider_settings_reject_raw_api_keys_and_headers() {
    for provider in [
        json!({"mode":"libretranslate","url":"http://127.0.0.1:5000","api_key":"sk-test"}),
        json!({"mode":"libretranslate","url":"http://127.0.0.1:5000","headers":{"Authorization":"Bearer fake"}}),
        json!({"mode":"libretranslate","url":"http://127.0.0.1:5000","extra_env":{"SECRET":"fake"}}),
    ] {
        let error = LaunchSettings::from_json_value(Some(&json!({
            "binary_path": "/tmp/translator-mcp",
            "provider": provider
        })))
        .expect_err("unsafe provider setting should be rejected");

        assert_eq!(error.code, DiagnosticCode::UnsafeLaunchConfiguration);
    }
}

#[test]
fn provider_settings_reject_non_local_url_without_allow_remote() {
    let error = LaunchSettings::from_json_value(Some(&json!({
        "binary_path": "/tmp/translator-mcp",
        "provider": {
            "mode": "libretranslate",
            "url": "https://translations.example.invalid",
            "allow_remote": false
        }
    })))
    .expect_err("non-local provider without allow_remote should be rejected");

    assert_eq!(error.code, DiagnosticCode::UnsafeLaunchConfiguration);
}

#[test]
fn provider_settings_accept_non_local_url_with_allow_remote() {
    let settings = LaunchSettings::from_json_value(Some(&json!({
        "binary_path": "/tmp/translator-mcp",
        "provider": {
            "mode": "libretranslate",
            "url": "https://translations.example.invalid",
            "allow_remote": true
        }
    })))
    .expect("allowlisted non-local provider setting should parse");

    assert_eq!(
        settings.provider_env().last().map(|entry| entry.1.as_str()),
        Some("true")
    );
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
        "zed-extension-provider-{case}-{}-{nanos}",
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
