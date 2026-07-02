#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    InvalidInput,
    UnsupportedLanguagePair,
    UnsupportedFileType,
    FileTooLarge,
    FileNotFound,
    PathNotAllowed,
    NonUtf8Input,
    NoTranslatableSegments,
    SecretDetected,
    ProviderNotConfigured,
    RemoteConfirmationRequired,
    ProviderFailed,
    ProviderTimeout,
    InternalError,
}

impl ErrorCode {
    pub const ALL: [ErrorCode; 14] = [
        ErrorCode::InvalidInput,
        ErrorCode::UnsupportedLanguagePair,
        ErrorCode::UnsupportedFileType,
        ErrorCode::FileTooLarge,
        ErrorCode::FileNotFound,
        ErrorCode::PathNotAllowed,
        ErrorCode::NonUtf8Input,
        ErrorCode::NoTranslatableSegments,
        ErrorCode::SecretDetected,
        ErrorCode::ProviderNotConfigured,
        ErrorCode::RemoteConfirmationRequired,
        ErrorCode::ProviderFailed,
        ErrorCode::ProviderTimeout,
        ErrorCode::InternalError,
    ];

    pub const fn as_str(&self) -> &'static str {
        match self {
            ErrorCode::InvalidInput => "INVALID_INPUT",
            ErrorCode::UnsupportedLanguagePair => "UNSUPPORTED_LANGUAGE_PAIR",
            ErrorCode::UnsupportedFileType => "UNSUPPORTED_FILE_TYPE",
            ErrorCode::FileTooLarge => "FILE_TOO_LARGE",
            ErrorCode::FileNotFound => "FILE_NOT_FOUND",
            ErrorCode::PathNotAllowed => "PATH_NOT_ALLOWED",
            ErrorCode::NonUtf8Input => "NON_UTF8_INPUT",
            ErrorCode::NoTranslatableSegments => "NO_TRANSLATABLE_SEGMENTS",
            ErrorCode::SecretDetected => "SECRET_DETECTED",
            ErrorCode::ProviderNotConfigured => "PROVIDER_NOT_CONFIGURED",
            ErrorCode::RemoteConfirmationRequired => "REMOTE_CONFIRMATION_REQUIRED",
            ErrorCode::ProviderFailed => "PROVIDER_FAILED",
            ErrorCode::ProviderTimeout => "PROVIDER_TIMEOUT",
            ErrorCode::InternalError => "INTERNAL_ERROR",
        }
    }
}
