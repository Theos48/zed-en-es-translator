use std::fs;
use std::path::Path;

const MANIFEST_PATH: &str = "extension.toml";

#[test]
fn manifest_has_required_metadata() {
    let manifest = manifest();

    assert_eq!(value_for_key(&manifest, "id"), Some("\"en-es-translator\""));
    assert_eq!(
        value_for_key(&manifest, "name"),
        Some("\"English to Spanish Translator\"")
    );
    assert_eq!(value_for_key(&manifest, "version"), Some("\"0.0.1\""));
    assert_eq!(value_for_key(&manifest, "schema_version"), Some("1"));
    assert_eq!(value_for_key(&manifest, "authors"), Some("[\"theos\"]"));
    assert_eq!(
        value_for_key(&manifest, "description"),
        Some("\"Local English to Spanish translator MCP wrapper.\"")
    );
}

#[test]
fn manifest_declares_exactly_one_context_server() {
    let manifest = manifest();

    assert_eq!(context_server_tables(&manifest), vec!["translator-en-es"]);
}

#[test]
fn manifest_context_server_matches_launch_contract() {
    let manifest = manifest();

    assert!(manifest.contains("[context_servers.translator-en-es]"));
}

#[test]
fn manifest_does_not_declare_out_of_scope_capabilities() {
    let manifest = manifest();

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

fn manifest() -> String {
    fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(MANIFEST_PATH))
        .expect("extension manifest should be readable")
}

fn value_for_key<'a>(manifest: &'a str, key: &str) -> Option<&'a str> {
    manifest.lines().find_map(|line| {
        let (candidate, value) = line.split_once('=')?;
        (candidate.trim() == key).then(|| value.trim())
    })
}

fn context_server_tables(manifest: &str) -> Vec<&str> {
    manifest
        .lines()
        .filter_map(|line| {
            line.trim()
                .strip_prefix("[context_servers.")
                .and_then(|rest| rest.strip_suffix(']'))
        })
        .collect()
}
