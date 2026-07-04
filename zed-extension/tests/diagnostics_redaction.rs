use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use zed_en_es_translator_extension::diagnostics::{
    corrective_action, redact_sensitive, DiagnosticCode, DiagnosticEvent, DiagnosticPhase,
};
use zed_en_es_translator_extension::launch::{
    artifact_status, diagnostic_for_artifact_status, PreparedArtifactStatus,
};

#[test]
fn redaction_removes_source_and_translation_samples() {
    let message = DiagnosticEvent::new(
        DiagnosticPhase::ServerRuntime,
        DiagnosticCode::InternalExtensionError,
        "source=Read the docs. translated=Lee la documentacion.",
    )
    .to_user_message();

    assert!(!message.contains("Read the docs."));
    assert!(!message.contains("Lee la documentacion."));
}

#[test]
fn redaction_removes_arbitrary_quoted_source_and_translation_assignments() {
    let source = "Please inspect the generated changelog.";
    let translated = "Revisa la salida generada antes de publicar.";
    let message = DiagnosticEvent::new(
        DiagnosticPhase::ServerRuntime,
        DiagnosticCode::InternalExtensionError,
        format!("source_text=\"{source}\" translated_text=\"{translated}\""),
    )
    .to_user_message();

    assert!(!message.contains(source));
    assert!(!message.contains(translated));
}

#[test]
fn redaction_removes_secrets_from_arbitrary_sentences_not_in_the_fixed_pair() {
    let redacted = redact_sensitive(
        "Startup log: Authorization: Bearer another_fake_token for /home/theos/other/app",
    );

    assert!(!redacted.contains("another_fake_token"));
    assert!(!redacted.contains("/home/theos/other"));
}

#[test]
fn redaction_removes_tokens_env_dumps_urls_and_full_paths() {
    let redacted = redact_sensitive(
        "Authorization: Bearer fake_token OPENAI_API_KEY=super_sensitive_value PATH=/home/theos/bin url=https://example.invalid /home/theos/project/file.md",
    );

    assert!(!redacted.contains("fake_token"));
    assert!(!redacted.contains("super_sensitive_value"));
    assert!(!redacted.contains("/home/theos"));
    assert!(!redacted.contains("https://example.invalid"));
}

#[test]
fn missing_binary_path_diagnostic_is_actionable() {
    let action = corrective_action(DiagnosticCode::BinaryPathNotConfigured);

    assert!(action.contains("make zed-extension-prepare"));
}

#[test]
fn artifact_status_reports_missing_artifact() {
    let path = unique_temp_dir("missing").join("translator-mcp");

    assert_eq!(artifact_status(&path), PreparedArtifactStatus::Missing);
}

#[test]
fn artifact_status_reports_non_executable_artifact() {
    let dir = unique_temp_dir("not-executable");
    fs::create_dir_all(&dir).expect("temp directory should be created");
    let path = dir.join("translator-mcp");
    fs::write(&path, "not executable").expect("artifact should be written");

    assert_eq!(
        artifact_status(&path),
        PreparedArtifactStatus::NotExecutable
    );
}

#[test]
fn stale_and_incompatible_artifacts_share_redacted_category() {
    for status in [
        PreparedArtifactStatus::Stale,
        PreparedArtifactStatus::IncompatibleCheckout,
        PreparedArtifactStatus::FailedOnStart,
    ] {
        let diagnostic = diagnostic_for_artifact_status(status);

        assert_eq!(diagnostic.code, DiagnosticCode::BinaryStaleOrIncompatible);
    }
}

#[test]
fn artifact_diagnostics_do_not_echo_sensitive_path() {
    let diagnostic = DiagnosticEvent::new(
        DiagnosticPhase::ArtifactValidation,
        DiagnosticCode::BinaryNotFound,
        "missing /home/theos/dev/private/target/release/translator-mcp",
    );

    assert!(!diagnostic.to_user_message().contains("/home/theos/dev"));
}

fn unique_temp_dir(case: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "zed-extension-diagnostics-{case}-{}-{nanos}",
        std::process::id()
    ))
}
