use crate::{ErrorCode, TranslateFailure};

pub fn redact_failure(failure: TranslateFailure) -> TranslateFailure {
    TranslateFailure::new(failure.code, default_redacted_message(failure.code))
}

pub fn redact_text(input: &str) -> String {
    let lower = input.to_ascii_lowercase();
    if lower.contains("bearer ")
        || lower.contains("api_key")
        || lower.contains("token")
        || lower.contains("-----begin")
        || lower.contains("/home/")
        || lower.contains("\\users\\")
    {
        "[REDACTED]".to_string()
    } else {
        input.to_string()
    }
}

fn default_redacted_message(code: ErrorCode) -> &'static str {
    match code {
        ErrorCode::InvalidInput => "Invalid input.",
        ErrorCode::UnsupportedLanguagePair => "Unsupported language pair.",
        ErrorCode::UnsupportedFileType => "Unsupported file type.",
        ErrorCode::FileTooLarge => "The input exceeds the configured size limit.",
        ErrorCode::FileNotFound => "The requested file was not found.",
        ErrorCode::PathNotAllowed => "The requested path is not allowed.",
        ErrorCode::NonUtf8Input => "The input must be UTF-8 text.",
        ErrorCode::NoTranslatableSegments => "No translatable segments were found.",
        ErrorCode::SecretDetected => "Potential secret content was detected.",
        ErrorCode::ProviderNotConfigured => "The provider is not configured for this request.",
        ErrorCode::RemoteConfirmationRequired => {
            "Remote provider confirmation is required for this request."
        }
        ErrorCode::ProviderFailed => "The provider failed.",
        ErrorCode::ProviderTimeout => "The provider timed out.",
        ErrorCode::InternalError => "An internal error occurred.",
    }
}
