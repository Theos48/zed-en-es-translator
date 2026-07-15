use translator_core::{
    ProviderConfiguration, ProviderLocality, ProviderMode, ProviderTarget, TranslateFailure,
};

#[test]
fn missing_provider_mode_resolves_to_mock() {
    let config = ProviderConfiguration::from_values(None, None, None, None).expect("mock config");

    assert_eq!(config.mode, ProviderMode::Mock);
}

#[test]
fn parses_loopback_libretranslate_target() {
    let config = ProviderConfiguration::from_values(
        Some("libretranslate"),
        Some("http://127.0.0.1:5000"),
        None,
        Some("false"),
    )
    .expect("local config");

    assert_eq!(
        config.target.as_ref().map(ProviderTarget::locality),
        Some(ProviderLocality::Local)
    );
}

#[test]
fn rejects_non_operational_libretranslate_remote_target() {
    let err = ProviderConfiguration::from_values(
        Some("libretranslate"),
        Some("https://translations.example.invalid"),
        Some("TRANSLATOR_TEST_API_KEY"),
        Some("true"),
    )
    .expect_err("remote LibreTranslate config must fail");

    assert_provider_not_configured(err);
}

#[test]
fn rejects_unknown_provider_mode() {
    let err = ProviderConfiguration::from_values(Some("other"), None, None, None)
        .expect_err("unknown mode should fail");

    assert_provider_not_configured(err);
}

#[test]
fn rejects_embedded_credentials_in_target_url() {
    let err = ProviderConfiguration::from_values(
        Some("libretranslate"),
        Some("https://user:pass@example.invalid"),
        None,
        Some("true"),
    )
    .expect_err("embedded credentials should fail");

    assert_provider_not_configured(err);
}

#[test]
fn rejects_api_key_env_that_looks_like_secret_value() {
    let err = ProviderConfiguration::from_values(
        Some("libretranslate"),
        Some("http://localhost:5000"),
        Some("sk-test123456"),
        None,
    )
    .expect_err("raw secret value should fail");

    assert_provider_not_configured(err);
}

fn assert_provider_not_configured(err: TranslateFailure) {
    assert_eq!(err.code, translator_core::ErrorCode::ProviderNotConfigured);
}
