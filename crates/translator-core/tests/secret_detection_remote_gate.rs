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

#[test]
fn blocks_common_private_key_headers_before_remote_processing() {
    for header in [
        "-----BEGIN RSA PRIVATE KEY-----",
        "-----BEGIN EC PRIVATE KEY-----",
        "-----BEGIN OPENSSH PRIVATE KEY-----",
        "-----BEGIN DSA PRIVATE KEY-----",
    ] {
        let err = check_remote_provider_gate(header, RemoteProviderState::ConfiguredButUnconfirmed)
            .expect_err("private key header should block");

        assert_eq!(err.code, ErrorCode::SecretDetected, "header: {header}");
    }
}

#[test]
fn blocks_common_config_secret_patterns_before_remote_processing() {
    for secret in [
        "PASSWORD=fake_password",
        "SECRET=fake_secret",
        "CLIENT_SECRET=fake_client_secret",
        "AWS_SECRET_ACCESS_KEY=fake_aws_secret",
        "X-Auth-Token: fake_token",
        "id_token: fake_id_token",
    ] {
        let err = check_remote_provider_gate(secret, RemoteProviderState::ConfiguredButUnconfirmed)
            .expect_err("config secret should block");

        assert_eq!(err.code, ErrorCode::SecretDetected, "secret: {secret}");
    }
}
