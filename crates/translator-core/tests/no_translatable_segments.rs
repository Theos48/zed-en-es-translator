use std::fs;
use std::path::{Path, PathBuf};

use translator_core::{translate_file, translate_text, ErrorCode, MockProvider};

fn temp_case(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "zed_translator_no_segments_{}_{}_{}",
        name,
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

fn write_file(path: &Path, content: &str) {
    fs::write(path, content).expect("write file");
}

#[test]
fn blank_text_file_returns_no_translatable_segments() {
    let workspace = temp_case("blank_txt");
    let file = workspace.join("blank.txt");
    write_file(&file, " \n\t\n");

    let err = translate_file(
        "blank.txt",
        workspace.to_str().unwrap(),
        &MockProvider::new(),
    )
    .expect_err("blank file should fail");

    assert_eq!(err.code, ErrorCode::NoTranslatableSegments);
}

#[test]
fn protected_only_markdown_returns_no_translatable_segments() {
    let workspace = temp_case("protected_markdown");
    let file = workspace.join("code_only.md");
    write_file(&file, "```rust\nfn main() {}\n```\n");

    let err = translate_file(
        "code_only.md",
        workspace.to_str().unwrap(),
        &MockProvider::new(),
    )
    .expect_err("protected-only markdown should fail");

    assert_eq!(err.code, ErrorCode::NoTranslatableSegments);
}

#[test]
fn direct_ambiguous_text_is_still_preserved_as_success() {
    let success = translate_text(r#"fn main() { println!("hi"); }"#, &MockProvider::new())
        .expect("ambiguous direct text is preserved");

    assert_eq!(success.translated_text, r#"fn main() { println!("hi"); }"#);
}
