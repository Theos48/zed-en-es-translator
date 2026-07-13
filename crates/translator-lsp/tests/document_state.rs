use std::str::FromStr;

use lsp_types::{Position, Range, Uri};
use translator_core::InputKind;
use translator_lsp::state::{DocumentStore, TranslationPreview};

fn uri() -> Uri {
    Uri::from_str("file:///workspace/readme.md").expect("valid URI")
}

#[test]
fn opens_changes_and_closes_a_full_sync_document() {
    let mut store = DocumentStore::default();
    store
        .open(uri(), 1, "markdown", "Read the docs.".to_string())
        .expect("open");

    let snapshot = store.get(&uri()).expect("snapshot");
    assert_eq!(snapshot.version(), 1);
    assert_eq!(snapshot.input_kind(), InputKind::Markdown);
    assert_eq!(snapshot.text(), "Read the docs.");

    store
        .change(&uri(), 2, "Open the file.".to_string())
        .expect("full change");
    assert_eq!(store.get(&uri()).expect("changed").version(), 2);

    store.close(&uri());
    assert!(store.get(&uri()).is_none());
}

#[test]
fn rejects_unsupported_languages_and_version_regressions() {
    let mut store = DocumentStore::default();
    assert!(store
        .open(uri(), 1, "rust", "fn main() {}".to_string())
        .is_err());

    store
        .open(uri(), 2, "markdown", "Read the docs.".to_string())
        .expect("open");
    assert!(store
        .change(&uri(), 2, "Open the file.".to_string())
        .is_err());
    assert_eq!(store.get(&uri()).expect("original").version(), 2);
}

#[test]
fn document_changes_and_close_invalidate_previews() {
    let mut store = DocumentStore::default();
    store
        .open(uri(), 1, "markdown", "Read the docs.".to_string())
        .expect("open");
    store.set_preview(TranslationPreview::new(
        uri(),
        1,
        Range::new(Position::new(0, 0), Position::new(0, 14)),
        "Lee la documentacion.".to_string(),
        InputKind::Markdown,
    ));
    assert!(store.preview(&uri()).is_some());

    store
        .change(&uri(), 2, "Open the file.".to_string())
        .expect("change");
    assert!(store.preview(&uri()).is_none());

    store.set_preview(TranslationPreview::new(
        uri(),
        2,
        Range::new(Position::new(0, 0), Position::new(0, 14)),
        "Abre el archivo.".to_string(),
        InputKind::Markdown,
    ));
    store.close(&uri());
    assert!(store.preview(&uri()).is_none());
}

#[test]
fn debug_output_redacts_document_and_preview_content() {
    let source = "SOURCE_SECRET_123";
    let translated = "TRANSLATED_SECRET_456";
    let mut store = DocumentStore::default();
    store
        .open(uri(), 1, "markdown", source.to_string())
        .expect("open");
    store.set_preview(TranslationPreview::new(
        uri(),
        1,
        Range::new(Position::new(0, 0), Position::new(0, 1)),
        translated.to_string(),
        InputKind::Markdown,
    ));

    let debug = format!("{store:?}");
    assert!(!debug.contains(source));
    assert!(!debug.contains(translated));
    assert!(!debug.contains("/workspace/readme.md"));
}
