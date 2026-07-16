use zed_en_es_translator_extension::acquisition::AcquisitionError;
use zed_en_es_translator_extension::diagnostics::{acquisition_message, redact_sensitive};

#[test]
fn acquisition_messages_are_stable_and_never_recommend_terminal_setup() {
    for error in [
        AcquisitionError::UnsupportedPlatform,
        AcquisitionError::Busy,
        AcquisitionError::DownloadFailed,
        AcquisitionError::InvalidPackage,
        AcquisitionError::StorageFailed,
    ] {
        let message = acquisition_message(error);
        assert!(message.len() <= 128);
        for forbidden in ["/home/", "http", "make ", "cargo", "docker", "binary_path"] {
            assert!(!message.to_ascii_lowercase().contains(forbidden));
        }
    }
}

#[test]
fn internal_diagnostics_redact_content_credentials_urls_and_paths() {
    let message = redact_sensitive(
        "source_text=private translated_text=privado TOKEN=secret https://example.invalid /home/user/file",
    );

    for forbidden in [
        "private",
        "privado",
        "secret",
        "example.invalid",
        "/home/user",
    ] {
        assert!(!message.contains(forbidden));
    }
}
