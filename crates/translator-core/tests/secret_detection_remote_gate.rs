use translator_core::{check_remote_provider_gate, ErrorCode, RemoteProviderState};

#[test]
fn blocks_api_key_before_remote_processing() {
    let err = check_remote_provider_gate(
        "API_KEY=fake_test_key_123456",
        RemoteProviderState::ConfiguredButUnconfirmed,
    )
    .expect_err("secret should block before confirmation handling");

    assert_eq!(err.code, ErrorCode::SecretDetected);
}

#[test]
fn blocks_bearer_token_before_remote_processing() {
    let err = check_remote_provider_gate(
        "Authorization: Bearer fake_test_bearer_token",
        RemoteProviderState::ConfiguredButUnconfirmed,
    )
    .expect_err("bearer token should block");

    assert_eq!(err.code, ErrorCode::SecretDetected);
}

#[test]
fn blocks_private_key_header_before_remote_processing() {
    let err = check_remote_provider_gate(
        "-----BEGIN PRIVATE KEY-----",
        RemoteProviderState::ConfiguredButUnconfirmed,
    )
    .expect_err("private key header should block");

    assert_eq!(err.code, ErrorCode::SecretDetected);
}
