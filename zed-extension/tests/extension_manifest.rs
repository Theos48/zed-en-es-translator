use std::fs;
use std::path::Path;

const MANIFEST_PATH: &str = "extension.toml";

#[test]
fn manifest_has_required_metadata() {
    let manifest = parsed_manifest();

    assert_eq!(manifest["id"].as_str(), Some("en-es-translator"));
    assert_eq!(
        manifest["name"].as_str(),
        Some("English to Spanish Translator")
    );
    assert_eq!(manifest["version"].as_str(), Some("0.0.1"));
    assert_eq!(manifest["schema_version"].as_integer(), Some(1));
    assert_eq!(
        manifest["authors"].as_array().map(|authors| authors
            .iter()
            .filter_map(toml::Value::as_str)
            .collect::<Vec<_>>()),
        Some(vec!["theos"])
    );
    assert_eq!(
        manifest["description"].as_str(),
        Some("Local English to Spanish translator MCP wrapper.")
    );
}

#[test]
fn manifest_declares_exactly_one_context_server() {
    let manifest = parsed_manifest();

    assert_eq!(context_server_ids(&manifest), vec!["translator-en-es"]);
}

#[test]
fn manifest_context_server_matches_launch_contract() {
    let manifest = manifest_text();

    assert!(manifest.contains("[context_servers.translator-en-es]"));
}

#[test]
fn manifest_does_not_declare_out_of_scope_capabilities() {
    let manifest = manifest_text();

    for forbidden in [
        "[languages.",
        "[themes.",
        "[grammars.",
        "[snippets.",
        "[debug_adapters.",
        "provider",
        "api_key",
        "base_url",
        "/home/",
    ] {
        assert!(
            !manifest.contains(forbidden),
            "manifest should not contain forbidden fragment {forbidden}"
        );
    }
}

fn manifest_text() -> String {
    fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(MANIFEST_PATH))
        .expect("extension manifest should be readable")
}

fn parsed_manifest() -> toml::Value {
    manifest_text()
        .parse()
        .expect("extension manifest should be valid TOML")
}

fn context_server_ids(manifest: &toml::Value) -> Vec<&str> {
    manifest
        .get("context_servers")
        .and_then(toml::Value::as_table)
        .map(|table| table.keys().map(String::as_str).collect())
        .unwrap_or_default()
}
