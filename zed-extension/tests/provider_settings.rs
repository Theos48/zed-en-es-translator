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
fn provider_settings_reject_non_local_libretranslate_even_with_allow_remote() {
    let error = LaunchSettings::from_json_value(Some(&json!({
        "binary_path": "/tmp/translator-mcp",
        "provider": {
            "mode": "libretranslate",
            "url": "https://translations.example.invalid",
            "allow_remote": true
        }
    })))
    .expect_err("non-local LibreTranslate is outside the operational matrix");

    assert_eq!(error.code, DiagnosticCode::UnsafeLaunchConfiguration);
}

#[test]
fn exact_local_profile_emits_only_applicable_environment_entries() {
    let settings = LaunchSettings::from_json_value(Some(&json!({
        "provider": {
            "mode": "libretranslate",
            "url": "http://127.0.0.1:5000"
        }
    })))
    .expect("exact local profile");

    assert_eq!(
        settings.provider_env(),
        vec![
            (
                "TRANSLATOR_PROVIDER".to_string(),
                "libretranslate".to_string()
            ),
            (
                "TRANSLATOR_PROVIDER_URL".to_string(),
                "http://127.0.0.1:5000".to_string()
            ),
        ]
    );
}

#[test]
fn exact_azure_profile_emits_reference_name_but_no_url_or_secret_value() {
    let raw_test_value = "fake-parent-secret-value";
    let settings = LaunchSettings::from_json_value(Some(&json!({
        "provider": {
            "mode": "azure_translator",
            "api_key_env": "AZURE_TRANSLATOR_KEY",
            "allow_remote": true
        }
    })))
    .expect("exact Azure profile");

    let environment = settings.provider_env();

    assert_eq!(
        environment,
        vec![
            (
                "TRANSLATOR_PROVIDER".to_string(),
                "azure_translator".to_string()
            ),
            (
                "TRANSLATOR_PROVIDER_API_KEY_ENV".to_string(),
                "AZURE_TRANSLATOR_KEY".to_string()
            ),
            (
                "TRANSLATOR_ALLOW_REMOTE_PROVIDER".to_string(),
                "true".to_string()
            ),
        ]
    );
    assert!(!format!("{environment:?}").contains(raw_test_value));
}

#[test]
fn provider_matrix_rejects_mode_inapplicable_values() {
    for provider in [
        json!({"mode":"mock","url":"http://127.0.0.1:5000"}),
        json!({"mode":"mock","api_key_env":"SAFE_KEY_NAME"}),
        json!({"mode":"mock","allow_remote":true}),
        json!({"mode":"libretranslate"}),
        json!({"mode":"libretranslate","url":"http://localhost:5000"}),
        json!({"mode":"libretranslate","url":"http://127.0.0.1:5000","api_key_env":"SAFE_KEY_NAME"}),
        json!({"mode":"libretranslate","url":"http://127.0.0.1:5000","allow_remote":true}),
        json!({"mode":"azure_translator","allow_remote":true}),
        json!({"mode":"azure_translator","api_key_env":"AZURE_TRANSLATOR_KEY"}),
        json!({"mode":"azure_translator","url":"https://api.cognitive.microsofttranslator.com","api_key_env":"AZURE_TRANSLATOR_KEY","allow_remote":true}),
    ] {
        let error = LaunchSettings::from_json_value(Some(&json!({"provider":provider})))
            .expect_err("invalid matrix entry must fail closed");

        assert_eq!(error.code, DiagnosticCode::UnsafeLaunchConfiguration);
    }
}

#[test]
fn duplicate_provider_environment_entries_are_rejected() {
    let error = LaunchSettings::from_parts(
        None,
        zed_en_es_translator_extension::settings::CommandSettingsInput::new(
            None,
            Vec::new(),
            vec![
                ("TRANSLATOR_PROVIDER".to_string(), "mock".to_string()),
                ("TRANSLATOR_PROVIDER".to_string(), "mock".to_string()),
            ],
        ),
    )
    .expect_err("duplicates must fail closed");

    assert_eq!(error.code, DiagnosticCode::UnsafeLaunchConfiguration);
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
