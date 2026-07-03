mod common;

#[test]
fn translate_file_rejects_traversal() {
    let workspace = common::temp_case("traversal");

    let value = common::translate_file_error_value(common::translate_file_params(
        &workspace,
        "../secret.md",
    ));

    common::assert_tool_error_code(&value, "PATH_NOT_ALLOWED");
}

#[test]
fn translate_file_rejects_binary_input() {
    let workspace = common::temp_case("binary");
    common::write_file(&workspace.join("bad.md"), [0, 1, 2, 3]);

    let value =
        common::translate_file_error_value(common::translate_file_params(&workspace, "bad.md"));

    common::assert_tool_error_code(&value, "NON_UTF8_INPUT");
}

#[test]
fn translate_file_rejects_nul_bytes() {
    let workspace = common::temp_case("nul");
    common::write_file(&workspace.join("nul.txt"), b"Read\0the docs.");

    let value =
        common::translate_file_error_value(common::translate_file_params(&workspace, "nul.txt"));

    common::assert_tool_error_code(&value, "NON_UTF8_INPUT");
}

#[test]
fn translate_file_rejects_non_utf8_input() {
    let workspace = common::temp_case("non_utf8");
    common::write_file(&workspace.join("bad.md"), [0xff, 0xfe, 0xfd]);

    let value =
        common::translate_file_error_value(common::translate_file_params(&workspace, "bad.md"));

    common::assert_tool_error_code(&value, "NON_UTF8_INPUT");
}

#[test]
fn translate_file_rejects_hidden_sensitive_files() {
    let workspace = common::temp_case("hidden_sensitive");
    common::write_file(&workspace.join(".env"), "TOKEN=secret");

    let value =
        common::translate_file_error_value(common::translate_file_params(&workspace, ".env"));

    common::assert_tool_error_code(&value, "PATH_NOT_ALLOWED");
}

#[test]
fn translate_file_rejects_credential_like_filenames() {
    let workspace = common::temp_case("credential_name");
    common::write_file(&workspace.join("secret.md"), "Read the docs.");

    let value =
        common::translate_file_error_value(common::translate_file_params(&workspace, "secret.md"));

    common::assert_tool_error_code(&value, "PATH_NOT_ALLOWED");
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

    let value =
        common::translate_file_error_value(common::translate_file_params(&workspace, "linked.md"));

    common::assert_tool_error_code(&value, "PATH_NOT_ALLOWED");
}
