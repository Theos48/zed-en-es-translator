use translator_core::{translate_text, MockProvider};

#[test]
fn translates_valid_direct_text_with_clean_success_output() {
    let provider = MockProvider::new();

    let success = translate_text(
        "Read the documentation before changing the code.",
        &provider,
    )
    .expect("valid text should translate");

    assert_eq!(
        success.translated_text,
        "Lee la documentacion antes de cambiar el codigo."
    );
}
