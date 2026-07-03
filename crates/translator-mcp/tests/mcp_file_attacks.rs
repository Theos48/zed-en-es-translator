mod common;

use serde_json::Value;
use translator_mcp::protocol::TranslateFileParams;
use translator_mcp::tools::TranslatorMcpServer;

#[test]
fn translate_file_rejects_traversal() {
    let workspace = common::temp_case("traversal");

    let value = translate_file_error_value(params(&workspace, "../secret.md"));

    assert_error_code(&value, "PATH_NOT_ALLOWED");
}

#[test]
fn translate_file_rejects_binary_input() {
    let workspace = common::temp_case("binary");
    common::write_file(&workspace.join("bad.md"), [0, 1, 2, 3]);

    let value = translate_file_error_value(params(&workspace, "bad.md"));

    assert_error_code(&value, "NON_UTF8_INPUT");
}

#[test]
fn translate_file_rejects_nul_bytes() {
    let workspace = common::temp_case("nul");
    common::write_file(&workspace.join("nul.txt"), b"Read\0the docs.");

    let value = translate_file_error_value(params(&workspace, "nul.txt"));

    assert_error_code(&value, "NON_UTF8_INPUT");
}

#[test]
fn translate_file_rejects_non_utf8_input() {
    let workspace = common::temp_case("non_utf8");
    common::write_file(&workspace.join("bad.md"), [0xff, 0xfe, 0xfd]);

    let value = translate_file_error_value(params(&workspace, "bad.md"));

    assert_error_code(&value, "NON_UTF8_INPUT");
}

#[test]
fn translate_file_rejects_hidden_sensitive_files() {
    let workspace = common::temp_case("hidden_sensitive");
    common::write_file(&workspace.join(".env"), "TOKEN=secret");

    let value = translate_file_error_value(params(&workspace, ".env"));

    assert_error_code(&value, "PATH_NOT_ALLOWED");
}

#[test]
fn translate_file_rejects_credential_like_filenames() {
    let workspace = common::temp_case("credential_name");
    common::write_file(&workspace.join("secret.md"), "Read the docs.");

    let value = translate_file_error_value(params(&workspace, "secret.md"));

    assert_error_code(&value, "PATH_NOT_ALLOWED");
}

#[cfg(unix)]
#[test]
fn translate_file_rejects_symlink_escape() {
    use std::os::unix::fs::symlink;

    let root = common::temp_case("symlink_escape");
    let workspace = root.join("workspace");
    let outside = root.join("outside.md");
    std::fs::create_dir_all(&workspace).expect("workspace");
    common::write_file(&outside, "Read the docs.");
    symlink(&outside, workspace.join("linked.md")).expect("symlink");

    let value = translate_file_error_value(params(&workspace, "linked.md"));

    assert_error_code(&value, "PATH_NOT_ALLOWED");
}

fn params(workspace: &std::path::Path, file_path: &str) -> TranslateFileParams {
    TranslateFileParams {
        workspace_root: workspace.to_string_lossy().into_owned(),
        file_path: file_path.to_string(),
        source_language: Some("en".to_string()),
        target_language: Some("es".to_string()),
        tone: Some("technical_neutral".to_string()),
        preserve_formatting: Some(true),
    }
}

fn translate_file_error_value(params: TranslateFileParams) -> Value {
    let result = TranslatorMcpServer::new().translate_file(params);
    serde_json::to_value(result).expect("serialize tool result")
}

fn assert_error_code(value: &Value, code: &str) {
    assert_eq!(value["isError"], true);
    assert_eq!(value["structuredContent"]["code"], code);
}
