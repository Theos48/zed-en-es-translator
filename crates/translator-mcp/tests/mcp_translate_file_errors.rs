mod common;

#[test]
fn translate_file_rejects_unsupported_extension() {
    let workspace = common::temp_case("unsupported_extension");
    common::write_file(&workspace.join("data.json"), "{}");

    let value =
        common::translate_file_error_value(common::translate_file_params(&workspace, "data.json"));

    common::assert_tool_error_code(&value, "UNSUPPORTED_FILE_TYPE");
}

#[test]
fn translate_file_rejects_protected_only_markdown() {
    let workspace = common::temp_case("protected_only_markdown");
    common::write_file(&workspace.join("code.md"), "```rust\nfn main() {}\n```\n");

    let value =
        common::translate_file_error_value(common::translate_file_params(&workspace, "code.md"));

    common::assert_tool_error_code(&value, "NO_TRANSLATABLE_SEGMENTS");
}
