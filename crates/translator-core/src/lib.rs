pub mod contract;
mod embedded_process;
mod embedded_protocol;
mod embedded_provider;
pub mod errors;
pub mod limits;
pub mod markdown;
pub mod provider;
pub mod redaction;
mod secrets;
pub mod workspace;

pub use contract::{
    validate_direct_text_input, validate_segments, InputKind, Language, Tone, TranslatableSegment,
    TranslateFailure, TranslateSuccess,
};
pub use embedded_process::{EmbeddedProcessRunner, EmbeddedRunnerLimits};
pub use embedded_provider::EmbeddedProcessProvider;
pub use errors::ErrorCode;
pub use limits::{
    MAX_INPUT_BYTES, MAX_OUTPUT_BYTES, MAX_SEGMENTS, MAX_SEGMENT_BYTES, PROVIDER_TIMEOUT_MS,
};
pub use provider::{
    ensure_provider_response_shape, MockProvider, Provider, ProviderRequest, ProviderResponse,
};
pub use redaction::{redact_failure, redact_text};
pub use workspace::{load_allowed_file, LoadedFile};

use std::ops::Range;

pub fn translate_text(
    source_text: &str,
    provider: &impl Provider,
) -> Result<TranslateSuccess, TranslateFailure> {
    translate_text_inner(source_text, provider).map_err(redact_failure)
}

fn translate_text_inner(
    source_text: &str,
    provider: &impl Provider,
) -> Result<TranslateSuccess, TranslateFailure> {
    validate_direct_text_input(source_text)?;

    if is_ambiguous_direct_text(source_text) {
        return TranslateSuccess::new(source_text);
    }

    let request = ProviderRequest::new(
        vec![source_text.to_string()],
        Language::English,
        Language::Spanish,
        Tone::TechnicalNeutral,
    )?;
    let response = provider.translate(&request)?;
    ensure_provider_response_shape(&request, &response)?;

    let translated_text = response
        .translated_segments
        .into_iter()
        .next()
        .ok_or_else(|| {
            TranslateFailure::new(
                ErrorCode::ProviderFailed,
                "Provider returned no translated segments.",
            )
        })?;

    TranslateSuccess::new(translated_text)
}

fn is_ambiguous_direct_text(source_text: &str) -> bool {
    let trimmed = source_text.trim();
    trimmed.contains("::")
        || trimmed.contains("=>")
        || trimmed.contains("```")
        || (trimmed.contains('(') && trimmed.contains(')') && trimmed.contains('"'))
}

pub fn translate_selection(
    document: &str,
    input_kind: InputKind,
    selection: Range<usize>,
    provider: &impl Provider,
) -> Result<TranslateSuccess, TranslateFailure> {
    translate_selection_inner(document, input_kind, selection, provider).map_err(redact_failure)
}

fn translate_selection_inner(
    document: &str,
    input_kind: InputKind,
    selection: Range<usize>,
    provider: &impl Provider,
) -> Result<TranslateSuccess, TranslateFailure> {
    let selected = document
        .get(selection.clone())
        .ok_or_else(|| TranslateFailure::invalid_input("Invalid selection range."))?;
    validate_direct_text_input(selected)?;
    if is_ambiguous_direct_text(selected)
        || (input_kind == InputKind::Markdown
            && markdown::selection_intersects_protected(document, selection))
    {
        return Err(TranslateFailure::invalid_input(
            "The selection contains protected or ambiguous content.",
        ));
    }

    let translated_text = markdown::translate_document(selected, input_kind, provider)?;
    TranslateSuccess::new(translated_text)
}

pub fn translate_file(
    file_path: &str,
    workspace_root: &str,
    provider: &impl Provider,
) -> Result<TranslateSuccess, TranslateFailure> {
    translate_file_inner(file_path, workspace_root, provider).map_err(redact_failure)
}

pub fn translate_document_snapshot(
    file_path: &str,
    workspace_root: &str,
    snapshot: &str,
    provider: &impl Provider,
) -> Result<TranslateSuccess, TranslateFailure> {
    translate_document_snapshot_inner(file_path, workspace_root, snapshot, provider)
        .map_err(redact_failure)
}

fn translate_document_snapshot_inner(
    file_path: &str,
    workspace_root: &str,
    snapshot: &str,
    provider: &impl Provider,
) -> Result<TranslateSuccess, TranslateFailure> {
    let loaded = load_allowed_file(file_path, workspace_root)?;
    validate_direct_text_input(snapshot)?;
    let translated_text = markdown::translate_document(snapshot, loaded.input_kind, provider)?;
    TranslateSuccess::new(translated_text)
}

fn translate_file_inner(
    file_path: &str,
    workspace_root: &str,
    provider: &impl Provider,
) -> Result<TranslateSuccess, TranslateFailure> {
    let loaded = load_allowed_file(file_path, workspace_root)?;
    let translated_text =
        markdown::translate_document(&loaded.content, loaded.input_kind, provider)?;

    TranslateSuccess::new(translated_text)
}
