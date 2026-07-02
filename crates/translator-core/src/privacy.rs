use crate::{secrets::contains_secret_pattern, ErrorCode, TranslateFailure};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteProviderState {
    Unconfigured,
    ConfiguredButUnconfirmed,
    ConfirmedButNotAllowlisted,
}

pub fn check_remote_provider_gate(
    source_text: &str,
    state: RemoteProviderState,
) -> Result<(), TranslateFailure> {
    if contains_obvious_secret(source_text) {
        return Err(TranslateFailure::new(
            ErrorCode::SecretDetected,
            "Potential secret content was detected.",
        ));
    }

    match state {
        RemoteProviderState::Unconfigured => Err(TranslateFailure::new(
            ErrorCode::ProviderNotConfigured,
            "The provider is not configured for this request.",
        )),
        RemoteProviderState::ConfiguredButUnconfirmed => Err(TranslateFailure::new(
            ErrorCode::RemoteConfirmationRequired,
            "Remote provider confirmation is required for this request.",
        )),
        RemoteProviderState::ConfirmedButNotAllowlisted => Err(TranslateFailure::new(
            ErrorCode::ProviderNotConfigured,
            "The provider is not allowlisted for this feature.",
        )),
    }
}

pub fn contains_obvious_secret(source_text: &str) -> bool {
    contains_secret_pattern(source_text)
}
