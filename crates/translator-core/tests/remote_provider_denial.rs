use translator_core::{check_remote_provider_gate, ErrorCode, RemoteProviderState};

#[test]
fn denies_unconfigured_remote_provider() {
    let err = check_remote_provider_gate("Read the docs.", RemoteProviderState::Unconfigured)
        .expect_err("unconfigured remote provider must be denied");

    assert_eq!(err.code, ErrorCode::ProviderNotConfigured);
}

#[test]
fn denies_configured_but_unconfirmed_remote_provider() {
    let err = check_remote_provider_gate(
        "Read the docs.",
        RemoteProviderState::ConfiguredButUnconfirmed,
    )
    .expect_err("unconfirmed remote provider must be denied");

    assert_eq!(err.code, ErrorCode::RemoteConfirmationRequired);
}

#[test]
fn denies_confirmed_but_not_allowlisted_remote_provider() {
    let err = check_remote_provider_gate(
        "Read the docs.",
        RemoteProviderState::ConfirmedButNotAllowlisted,
    )
    .expect_err("not-allowlisted remote provider must be denied");

    assert_eq!(err.code, ErrorCode::ProviderNotConfigured);
}
