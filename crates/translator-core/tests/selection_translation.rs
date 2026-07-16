use std::cell::Cell;

use translator_core::{
    translate_selection, ErrorCode, InputKind, MockProvider, Provider, ProviderRequest,
    ProviderResponse, TranslateFailure, MAX_INPUT_BYTES,
};

#[test]
fn translates_safe_plain_text_selection_without_mutating_source() {
    let source = String::from("Before. Read the docs. After.");
    let before = source.clone();
    let start = source.find("Read").expect("selection start");
    let end = start + "Read the docs.".len();

    let success = translate_selection(&source, InputKind::Text, start..end, &MockProvider::new())
        .expect("selection translation");

    assert_eq!(success.translated_text, "Lee la documentacion.");
    assert_eq!(source, before);
}

#[test]
fn rejects_blank_invalid_and_oversized_selection_ranges() {
    let source = "Read the docs.";
    for range in [0..0, 4..5, 0..source.len() + 1] {
        let error = translate_selection(source, InputKind::Text, range, &MockProvider::new())
            .expect_err("invalid selection must fail");
        assert_eq!(error.code, ErrorCode::InvalidInput);
    }

    let oversized = "x".repeat(MAX_INPUT_BYTES + 1);
    let error = translate_selection(
        &oversized,
        InputKind::Text,
        0..oversized.len(),
        &MockProvider::new(),
    )
    .expect_err("oversized selection must fail");
    assert_eq!(error.code, ErrorCode::FileTooLarge);
}

#[test]
fn rejects_markdown_selection_inside_or_crossing_protected_content() {
    let source = "Read `the docs` now.";
    for range in [6..14, 0..10] {
        let error = translate_selection(source, InputKind::Markdown, range, &MockProvider::new())
            .expect_err("protected selection must fail");
        assert_eq!(error.code, ErrorCode::InvalidInput);
    }
}

#[test]
fn rejects_ambiguous_code_like_selection_before_provider_contact() {
    let provider = CountingProvider::default();
    let source = "call(\"Read the docs.\")";

    let error = translate_selection(source, InputKind::Markdown, 0..source.len(), &provider)
        .expect_err("ambiguous selection must fail");

    assert_eq!(error.code, ErrorCode::InvalidInput);
    assert_eq!(provider.calls.get(), 0);
}

#[derive(Default)]
struct CountingProvider {
    calls: Cell<usize>,
}

impl Provider for CountingProvider {
    fn translate(&self, request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        self.calls.set(self.calls.get() + 1);
        Ok(ProviderResponse {
            translated_segments: request.segments.clone(),
        })
    }
}
