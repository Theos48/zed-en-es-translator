use std::fs;
use std::path::Path;

#[test]
fn manifest_registers_one_direct_language_server_and_no_manual_surface() {
    let text = manifest_text();
    let manifest: toml::Value = toml::from_str(&text).expect("extension manifest");

    assert_eq!(manifest["id"].as_str(), Some("en-es-translator"));
    assert_eq!(manifest["version"].as_str(), Some("0.1.0"));
    assert_eq!(
        manifest["language_servers"]["en-es-translator"]["languages"]
            .as_array()
            .expect("languages"),
        &[
            toml::Value::String("Markdown".to_string()),
            toml::Value::String("Plain Text".to_string())
        ]
    );
    assert!(manifest.get("context_servers").is_none());
    for forbidden in [
        "binary_path",
        "provider",
        "api_key",
        "base_url",
        "/home/",
        "make ",
        "docker",
        "cargo",
    ] {
        assert!(!text.to_ascii_lowercase().contains(forbidden));
    }
}

fn manifest_text() -> String {
    fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("extension.toml"))
        .expect("extension manifest")
}
