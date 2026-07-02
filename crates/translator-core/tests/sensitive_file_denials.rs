use std::fs;
use std::path::{Path, PathBuf};

use translator_core::{translate_file, ErrorCode, MockProvider};

fn temp_case() -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "zed_translator_sensitive_{}_{}",
        std::process::id(),
        unique_suffix()
    ));
    fs::create_dir_all(&root).expect("temp root");
    root
}

fn unique_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time")
        .as_nanos()
}

fn write_file(path: &Path) {
    fs::write(path, "API_KEY=secret").expect("write file");
}

#[test]
fn rejects_hidden_env_file_with_supported_extension() {
    let workspace = temp_case();
    write_file(&workspace.join(".env.md"));

    let err = translate_file(".env.md", workspace.to_str().unwrap(), &MockProvider::new())
        .expect_err("hidden env file should fail");

    assert_eq!(err.code, ErrorCode::PathNotAllowed);
}

#[test]
fn rejects_credential_like_supported_filename() {
    let workspace = temp_case();
    write_file(&workspace.join("credentials.markdown"));

    let err = translate_file(
        "credentials.markdown",
        workspace.to_str().unwrap(),
        &MockProvider::new(),
    )
    .expect_err("credential-like filename should fail");

    assert_eq!(err.code, ErrorCode::PathNotAllowed);
}
