use zed_en_es_translator_extension::diagnostics::DiagnosticCode;
use zed_en_es_translator_extension::settings::LaunchSettings;
use zed_extension_api::serde_json::json;

#[test]
fn embedded_mode_forwards_only_the_controlled_provider_selector() {
    let settings = LaunchSettings::from_json_value(Some(&json!({
        "provider": {"mode": "embedded_local"}
    })))
    .expect("embedded settings");

    assert_eq!(
        settings.provider_env(),
        vec![(
            "TRANSLATOR_PROVIDER".to_string(),
            "embedded_local".to_string()
        )]
    );
}

#[test]
fn embedded_mode_rejects_paths_downloads_and_inapplicable_provider_keys() {
    for provider in [
        json!({"mode":"embedded_local","url":"https://example.invalid/model"}),
        json!({"mode":"embedded_local","api_key_env":"SAFE_KEY_NAME"}),
        json!({"mode":"embedded_local","allow_remote":false}),
        json!({"mode":"embedded_local","artifact_path":"/private/model"}),
        json!({"mode":"embedded_local","download":true}),
    ] {
        let error = LaunchSettings::from_json_value(Some(&json!({"provider":provider})))
            .expect_err("embedded conflict must fail closed");

        assert_eq!(error.code, DiagnosticCode::UnsafeLaunchConfiguration);
    }
}
