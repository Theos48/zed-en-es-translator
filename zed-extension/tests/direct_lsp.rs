use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use zed_en_es_translator_extension::diagnostics::DiagnosticCode;
use zed_en_es_translator_extension::launch::{
    build_lsp_launch_profile, DIRECT_LSP_ID, TRANSLATOR_LSP_BINARY,
};
use zed_en_es_translator_extension::settings::{CommandSettingsInput, LaunchSettings};
use zed_extension_api::serde_json::json;

#[test]
fn manifest_registers_direct_server_for_markdown_and_plain_text() {
    let manifest: toml::Value =
        toml::from_str(include_str!("../extension.toml")).expect("manifest");
    let server = &manifest["language_servers"][DIRECT_LSP_ID];

    assert_eq!(
        server["name"].as_str(),
        Some("English to Spanish Translator")
    );
    assert_eq!(
        server["languages"].as_array().expect("languages"),
        &[
            toml::Value::String("Markdown".to_string()),
            toml::Value::String("Plain Text".to_string())
        ]
    );
}

#[test]
fn builds_empty_argument_lsp_command_with_only_allowlisted_environment() {
    let artifact = executable_artifact("direct-lsp");
    let settings = LaunchSettings::from_json_value(Some(&json!({
        "binary_path": artifact.display().to_string(),
        "provider": {"mode":"libretranslate","url":"http://127.0.0.1:5000","allow_remote":false}
    })))
    .expect("settings");

    let profile = build_lsp_launch_profile(DIRECT_LSP_ID, &settings).expect("profile");
    assert_eq!(profile.command, artifact.display().to_string());
    assert!(profile.args.is_empty());
    assert_eq!(profile.env.len(), 3);
    assert!(profile
        .env
        .iter()
        .all(|(key, _)| key.starts_with("TRANSLATOR_")));
}

#[test]
fn accepts_provider_configuration_from_zed_binary_environment() {
    let artifact = executable_artifact("direct-lsp-binary-env");
    let settings = LaunchSettings::from_parts(
        None,
        CommandSettingsInput::new(
            Some(artifact.display().to_string()),
            Vec::new(),
            vec![
                (
                    "TRANSLATOR_PROVIDER".to_string(),
                    "libretranslate".to_string(),
                ),
                (
                    "TRANSLATOR_PROVIDER_URL".to_string(),
                    "https://translations.example.invalid".to_string(),
                ),
                (
                    "TRANSLATOR_ALLOW_REMOTE_PROVIDER".to_string(),
                    "true".to_string(),
                ),
            ],
        ),
    )
    .expect("controlled Zed binary environment should parse");

    let profile = build_lsp_launch_profile(DIRECT_LSP_ID, &settings).expect("profile");
    assert_eq!(
        profile.env,
        vec![
            (
                "TRANSLATOR_PROVIDER".to_string(),
                "libretranslate".to_string(),
            ),
            (
                "TRANSLATOR_PROVIDER_URL".to_string(),
                "https://translations.example.invalid".to_string(),
            ),
            (
                "TRANSLATOR_ALLOW_REMOTE_PROVIDER".to_string(),
                "true".to_string(),
            ),
        ]
    );
}

#[test]
fn rejects_conflicting_nested_and_binary_environment_provider_configuration() {
    let error = LaunchSettings::from_parts(
        Some(&json!({"provider":{"mode":"mock"}})),
        CommandSettingsInput::new(
            Some("/tmp/translator-lsp".to_string()),
            Vec::new(),
            vec![
                (
                    "TRANSLATOR_PROVIDER".to_string(),
                    "libretranslate".to_string(),
                ),
                (
                    "TRANSLATOR_PROVIDER_URL".to_string(),
                    "https://translations.example.invalid".to_string(),
                ),
                (
                    "TRANSLATOR_ALLOW_REMOTE_PROVIDER".to_string(),
                    "true".to_string(),
                ),
            ],
        ),
    )
    .expect_err("conflicting provider channels should fail closed");

    assert_eq!(error.code, DiagnosticCode::UnsafeLaunchConfiguration);
    assert!(error
        .to_user_message()
        .contains("conflicting provider configuration"));
    assert!(!error
        .to_user_message()
        .contains("translations.example.invalid"));
}

#[test]
fn rejects_incomplete_or_invalid_controlled_provider_environment() {
    for env in [
        vec![(
            "TRANSLATOR_PROVIDER_URL".to_string(),
            "https://translations.example.invalid".to_string(),
        )],
        vec![
            (
                "TRANSLATOR_PROVIDER".to_string(),
                "libretranslate".to_string(),
            ),
            (
                "TRANSLATOR_PROVIDER_URL".to_string(),
                "https://translations.example.invalid".to_string(),
            ),
            (
                "TRANSLATOR_ALLOW_REMOTE_PROVIDER".to_string(),
                "sometimes".to_string(),
            ),
        ],
    ] {
        let error = LaunchSettings::from_parts(
            None,
            CommandSettingsInput::new(Some("/tmp/translator-lsp".to_string()), Vec::new(), env),
        )
        .expect_err("invalid controlled provider environment should fail closed");

        assert_eq!(error.code, DiagnosticCode::UnsafeLaunchConfiguration);
    }
}

#[test]
fn direct_configuration_failure_names_direct_recovery_command_and_binary() {
    let error = build_lsp_launch_profile(DIRECT_LSP_ID, &LaunchSettings::default())
        .expect_err("missing direct path should fail");
    let message = error.to_user_message();

    assert!(message.contains("make zed-direct-prepare"));
    assert!(message.contains("translator-lsp"));
    assert!(!message.contains("zed-extension-prepare"));
    assert!(!message.contains("translator-mcp"));
}

#[test]
fn rejects_wrong_binary_extra_arguments_and_redacts_unsafe_values() {
    let wrong =
        LaunchSettings::from_json_value(Some(&json!({"binary_path":"/tmp/translator-mcp"})))
            .expect("settings");
    let error = build_lsp_launch_profile(DIRECT_LSP_ID, &wrong).expect_err("wrong binary");
    assert_eq!(error.code, DiagnosticCode::BinaryStaleOrIncompatible);

    let secret = "SOURCE_SECRET_123";
    let extra = LaunchSettings::from_parts(
        None,
        CommandSettingsInput::new(
            Some(format!("/tmp/{TRANSLATOR_LSP_BINARY}")),
            vec![secret.to_string()],
            Vec::new(),
        ),
    )
    .expect_err("extra argument");
    assert!(!extra.to_user_message().contains(secret));
}

#[test]
fn provider_url_key_values_and_arbitrary_environment_never_enter_errors() {
    for (settings, forbidden) in [
        (
            json!({"binary_path":"/tmp/translator-lsp","provider":{"mode":"libretranslate","url":"https://private-provider.example.invalid","allow_remote":false}}),
            "private-provider",
        ),
        (
            json!({"binary_path":"/tmp/translator-lsp","provider":{"mode":"libretranslate","url":"http://127.0.0.1:5000","api_key_env":"not a valid key"}}),
            "not a valid key",
        ),
    ] {
        let error = LaunchSettings::from_json_value(Some(&settings)).expect_err("unsafe settings");
        assert!(!error.to_user_message().contains(forbidden));
    }

    let arbitrary = LaunchSettings::from_parts(
        None,
        CommandSettingsInput::new(
            Some("/tmp/translator-lsp".to_string()),
            Vec::new(),
            vec![("PRIVATE_TOKEN".to_string(), "PRIVATE_VALUE".to_string())],
        ),
    )
    .expect_err("arbitrary environment");
    assert!(!arbitrary.to_user_message().contains("PRIVATE_VALUE"));
}

fn executable_artifact(case: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let directory =
        std::env::temp_dir().join(format!("zed-direct-{case}-{}-{nanos}", std::process::id()));
    fs::create_dir_all(&directory).expect("directory");
    let path = directory.join(TRANSLATOR_LSP_BINARY);
    fs::write(&path, "#!/bin/sh\nexit 0\n").expect("artifact");
    make_executable(&path);
    path
}

#[cfg(unix)]
fn make_executable(path: &PathBuf) {
    let mut permissions = fs::metadata(path).expect("metadata").permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).expect("permissions");
}

#[cfg(not(unix))]
fn make_executable(_path: &PathBuf) {}
