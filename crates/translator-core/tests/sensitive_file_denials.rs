use translator_core::{translate_file, ErrorCode, MockProvider};

mod common;
use common::{temp_case, write_file};

#[test]
fn rejects_hidden_env_file_with_supported_extension() {
    let workspace = temp_case("sensitive_env");
    write_file(&workspace.join(".env.md"), "API_KEY=secret");

    let err = translate_file(".env.md", workspace.to_str().unwrap(), &MockProvider::new())
        .expect_err("hidden env file should fail");

    assert_eq!(err.code, ErrorCode::PathNotAllowed);
}

#[test]
fn rejects_credential_like_supported_filename() {
    let workspace = temp_case("sensitive_credentials");
    write_file(&workspace.join("credentials.markdown"), "API_KEY=secret");

    let err = translate_file(
        "credentials.markdown",
        workspace.to_str().unwrap(),
        &MockProvider::new(),
    )
    .expect_err("credential-like filename should fail");

    assert_eq!(err.code, ErrorCode::PathNotAllowed);
}
