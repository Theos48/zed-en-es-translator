use crate::errors::ErrorCode;
use crate::limits::{MAX_INPUT_BYTES, MAX_OUTPUT_BYTES, MAX_SEGMENTS, MAX_SEGMENT_BYTES};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Spanish,
}

impl Language {
    pub const fn as_str(self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Spanish => "es",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    TechnicalNeutral,
}

impl Tone {
    pub const fn as_str(self) -> &'static str {
        match self {
            Tone::TechnicalNeutral => "technical_neutral",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputKind {
    Text,
    Markdown,
}

impl InputKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            InputKind::Text => "text",
            InputKind::Markdown => "markdown",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranslateSuccess {
    pub translated_text: String,
}

impl TranslateSuccess {
    pub fn new(translated_text: impl Into<String>) -> Result<Self, TranslateFailure> {
        let translated_text = translated_text.into();
        if translated_text.len() > MAX_OUTPUT_BYTES {
            return Err(TranslateFailure::new(
                ErrorCode::ProviderFailed,
                "Provider output exceeds the configured size limit.",
            ));
        }
        Ok(Self { translated_text })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranslateFailure {
    pub code: ErrorCode,
    pub message: String,
}

impl TranslateFailure {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::InvalidInput, message)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranslatableSegment {
    pub id: usize,
    pub text: String,
}

impl TranslatableSegment {
    pub fn new(id: usize, text: impl Into<String>) -> Result<Self, TranslateFailure> {
        let text = text.into();
        if text.len() > MAX_SEGMENT_BYTES {
            return Err(TranslateFailure::new(
                ErrorCode::FileTooLarge,
                "A translatable segment exceeds the configured size limit.",
            ));
        }
        Ok(Self { id, text })
    }
}

pub fn validate_segments(segments: &[TranslatableSegment]) -> Result<(), TranslateFailure> {
    if segments.is_empty() {
        return Err(TranslateFailure::new(
            ErrorCode::NoTranslatableSegments,
            "No translatable segments were found.",
        ));
    }
    if segments.len() > MAX_SEGMENTS {
        return Err(TranslateFailure::new(
            ErrorCode::FileTooLarge,
            "The request contains too many translatable segments.",
        ));
    }
    for segment in segments {
        if segment.text.len() > MAX_SEGMENT_BYTES {
            return Err(TranslateFailure::new(
                ErrorCode::FileTooLarge,
                "A translatable segment exceeds the configured size limit.",
            ));
        }
    }
    Ok(())
}

pub fn validate_direct_text_input(text: &str) -> Result<(), TranslateFailure> {
    if text.trim().is_empty() {
        return Err(TranslateFailure::new(
            ErrorCode::InvalidInput,
            "Input text must not be empty.",
        ));
    }
    validate_input_size(text)
}

fn validate_input_size(text: &str) -> Result<(), TranslateFailure> {
    if text.len() > MAX_INPUT_BYTES {
        return Err(TranslateFailure::new(
            ErrorCode::FileTooLarge,
            "The input exceeds the configured size limit.",
        ));
    }
    Ok(())
}
