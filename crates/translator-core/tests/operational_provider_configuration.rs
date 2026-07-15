use translator_core::{
    ErrorCode, ProviderConfiguration, ProviderLocality, ProviderMode, ProviderTarget,
    TranslateFailure,
};

#[test]
fn absent_configuration_selects_mock_without_target_or_secret_reference() {
    let configuration =
        ProviderConfiguration::from_values(None, None, None, None).expect("mock configuration");

    assert_eq!(
        (
            configuration.mode,
            configuration.target,
            configuration.api_key_env
        ),
        (ProviderMode::Mock, None, None)
    );
}

#[test]
fn exact_local_profile_is_loopback_and_proxy_independent() {
    let configuration = ProviderConfiguration::from_values(
        Some("libretranslate"),
        Some("http://127.0.0.1:5000"),
        None,
        None,
    )
    .expect("local configuration");

    assert_eq!(
        configuration.target.as_ref().map(ProviderTarget::locality),
        Some(ProviderLocality::Local)
    );
    assert!(!configuration.uses_inherited_proxy_environment());
}

#[test]
fn exact_azure_profile_has_no_caller_controlled_target() {
    let configuration = ProviderConfiguration::from_values(
        Some("azure_translator"),
        None,
        Some("AZURE_TRANSLATOR_KEY"),
        Some("true"),
    )
    .expect("azure configuration");

    assert_eq!(configuration.mode, ProviderMode::AzureTranslator);
    assert!(configuration.target.is_none());
    assert_eq!(
        configuration.api_key_env.as_deref(),
        Some("AZURE_TRANSLATOR_KEY")
    );
}

#[test]
fn mode_matrix_rejects_incomplete_conflicting_and_inapplicable_values() {
    for (provider, url, key_reference, allow_remote) in [
        (Some(""), None, None, None),
        (Some("mock"), Some("http://127.0.0.1:5000"), None, None),
        (Some("mock"), None, Some("SAFE_KEY_NAME"), None),
        (Some("mock"), None, None, Some("true")),
        (Some("libretranslate"), None, None, None),
        (
            Some("libretranslate"),
            Some("http://localhost:5000"),
            None,
            None,
        ),
        (
            Some("libretranslate"),
            Some("http://127.0.0.1:5001"),
            None,
            None,
        ),
        (
            Some("libretranslate"),
            Some("http://127.0.0.1:5000/"),
            None,
            None,
        ),
        (
            Some("libretranslate"),
            Some("http://127.0.0.1:5000"),
            Some("SAFE_KEY_NAME"),
            None,
        ),
        (
            Some("libretranslate"),
            Some("http://127.0.0.1:5000"),
            None,
            Some("true"),
        ),
        (Some("azure_translator"), None, None, Some("true")),
        (
            Some("azure_translator"),
            Some("https://api.cognitive.microsofttranslator.com"),
            Some("AZURE_TRANSLATOR_KEY"),
            Some("true"),
        ),
        (
            Some("azure_translator"),
            None,
            Some("AZURE_TRANSLATOR_KEY"),
            None,
        ),
        (
            Some("azure_translator"),
            None,
            Some("AZURE_TRANSLATOR_KEY"),
            Some("1"),
        ),
    ] {
        assert_provider_not_configured(ProviderConfiguration::from_values(
            provider,
            url,
            key_reference,
            allow_remote,
        ));
    }
}

#[test]
fn key_reference_must_be_a_nonempty_environment_identifier() {
    for key_reference in ["", "9KEY", "AZURE-KEY", "sk-public-test-value"] {
        assert_provider_not_configured(ProviderConfiguration::from_values(
            Some("azure_translator"),
            None,
            Some(key_reference),
            Some("true"),
        ));
    }
}

fn assert_provider_not_configured(result: Result<ProviderConfiguration, TranslateFailure>) {
    let failure = result.expect_err("configuration must fail closed");
    assert_eq!(failure.code, ErrorCode::ProviderNotConfigured);
    assert!(!failure.message.contains("AZURE_TRANSLATOR_KEY"));
    assert!(!failure.message.contains("sk-public-test-value"));
}
