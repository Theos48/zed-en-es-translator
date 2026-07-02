use std::fs;
use std::path::PathBuf;

use translator_core::{translate_file, MockProvider};

fn temp_case() -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "zed_translator_no_mutation_{}_{}",
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

#[test]
fn translating_file_does_not_mutate_source_file() {
    let workspace = temp_case();
    let file = workspace.join("note.md");
    let original = "# Read the docs\n\nUse `cargo test`.\n";
    fs::write(&file, original).expect("write file");

    let success = translate_file("note.md", workspace.to_str().unwrap(), &MockProvider::new())
        .expect("translation should succeed");
    let after = fs::read_to_string(&file).expect("read file after translation");

    assert_ne!(success.translated_text, original);
    assert_eq!(after, original);
}
