use translator_core::{translate_file, translate_text, LibreTranslateProvider, ProviderTarget};

mod common;

use common::StubHttpServer;

#[test]
fn translates_direct_text_with_local_provider() {
    let server = StubHttpServer::new(r#"{"translatedText":["Lee la documentacion."]}"#);
    let provider = LibreTranslateProvider::new(
        ProviderTarget::parse(&server.url(), false).expect("target"),
        None,
    );

    let translated = translate_text("Read the docs.", &provider).expect("translation");

    assert_eq!(translated.translated_text, "Lee la documentacion.");
}

#[test]
fn translates_allowed_file_without_mutating_source() {
    let server = StubHttpServer::with_requests(2, r#"{"translatedText":["Abre el archivo."]}"#);
    let provider = LibreTranslateProvider::new(
        ProviderTarget::parse(&server.url(), false).expect("target"),
        None,
    );
    let workspace = temp_workspace("local-provider-file");
    let file = workspace.join("notes.md");
    let original = "# Notes\n\nOpen the file.\n\n```rust\nfn main() {}\n```\n";
    std::fs::write(&file, original).expect("write file");

    let translated = translate_file(
        "notes.md",
        workspace.to_str().expect("workspace"),
        &provider,
    )
    .expect("file translation");

    assert!(translated.translated_text.contains("Abre el archivo."));
    assert_eq!(std::fs::read_to_string(file).expect("read file"), original);
    for body in server.bodies(2) {
        let payload: serde_json::Value = serde_json::from_str(&body).expect("json body");
        assert!(payload.get("workspace").is_none());
        assert!(payload.get("workspace_root").is_none());
        assert!(payload.get("file_path").is_none());
        assert!(!body.contains("notes.md"));
        assert!(!body.contains(workspace.to_str().expect("workspace")));
    }
}

fn temp_workspace(case: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!(
        "zed_translator_{case}_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos()
    ));
    std::fs::create_dir_all(&root).expect("temp root");
    root
}
