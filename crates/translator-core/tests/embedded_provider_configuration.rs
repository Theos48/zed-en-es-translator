use translator_core::{
    ErrorCode, ProviderConfiguration, ProviderMode, ProviderSelection, TranslateFailure,
};

#[test]
fn embedded_local_accepts_only_the_provider_selector() {
    let configuration =
        ProviderConfiguration::from_values(Some("embedded_local"), None, None, None)
            .expect("embedded configuration");

    assert_eq!(configuration.mode, ProviderMode::EmbeddedLocal);
    assert!(configuration.target.is_none());
    assert!(configuration.api_key_env.is_none());
    assert!(!configuration.uses_inherited_proxy_environment());
}

#[test]
fn embedded_local_rejects_every_conflicting_controlled_key() {
    for (url, key_reference, allow_remote) in [
        (Some("http://127.0.0.1:5000"), None, None),
        (None, Some("SAFE_KEY_NAME"), None),
        (None, None, Some("false")),
        (None, None, Some("true")),
    ] {
        assert_not_configured(ProviderConfiguration::from_values(
            Some("embedded_local"),
            url,
            key_reference,
            allow_remote,
        ));
    }
}

#[test]
fn explicit_embedded_selection_fails_closed_when_no_set_is_ready() {
    let configuration =
        ProviderConfiguration::from_values(Some("embedded_local"), None, None, None)
            .expect("embedded configuration");

    let failure = ProviderSelection::from_configuration(configuration)
        .expect_err("missing installation must not fall back to Mock");

    assert_eq!(failure.code, ErrorCode::ProviderNotConfigured);
    assert!(!failure.message.contains("HOME"));
    assert!(!failure.message.contains("XDG"));
}

#[test]
fn absent_mode_still_selects_mock() {
    let configuration =
        ProviderConfiguration::from_values(None, None, None, None).expect("mock configuration");

    assert_eq!(configuration.mode, ProviderMode::Mock);
    assert!(matches!(
        ProviderSelection::from_configuration(configuration),
        Ok(ProviderSelection::Mock(_))
    ));
}

fn assert_not_configured(result: Result<ProviderConfiguration, TranslateFailure>) {
    let failure = result.expect_err("conflicting embedded configuration must fail");
    assert_eq!(failure.code, ErrorCode::ProviderNotConfigured);
}
