use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use crate::diagnostics::{
    diagnostic_with_action, direct_diagnostic_with_action, DiagnosticCode, DiagnosticEvent,
    DiagnosticPhase,
};
use crate::settings::LaunchSettings;

/// Single context server id declared by `extension.toml`.
pub const CONTEXT_SERVER_ID: &str = "translator-en-es";

/// Direct language server id declared by `extension.toml`.
pub const DIRECT_LSP_ID: &str = "en-es-translator";

/// Existing MCP server binary this wrapper is allowed to launch.
pub const TRANSLATOR_MCP_BINARY: &str = "translator-mcp";

/// Native direct-workflow binary this wrapper is allowed to launch.
pub const TRANSLATOR_LSP_BINARY: &str = "translator-lsp";

/// Controlled launch profile returned to Zed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchProfile {
    /// Direct executable path. This is never shell-split.
    pub command: String,
    /// Controlled argument vector. Empty for this feature.
    pub args: Vec<String>,
    /// Explicit environment allowlist.
    pub env: Vec<(String, String)>,
}

/// Local server artifact status before launch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreparedArtifactStatus {
    /// The configured path does not exist.
    Missing,
    /// The path exists but is not a regular file.
    NotFile,
    /// The artifact cannot be executed directly.
    NotExecutable,
    /// The artifact is from an older build.
    Stale,
    /// The artifact is not compatible with this checkout.
    IncompatibleCheckout,
    /// The artifact failed immediately after launch.
    FailedOnStart,
    /// The artifact is usable for direct launch.
    Usable,
}

/// Build the direct Zed command profile for `translator-en-es`.
pub fn build_launch_profile(
    context_server_id: &str,
    settings: &LaunchSettings,
) -> Result<LaunchProfile, DiagnosticEvent> {
    if context_server_id != CONTEXT_SERVER_ID {
        return Err(diagnostic_with_action(
            DiagnosticPhase::Configuration,
            DiagnosticCode::UnsupportedContextServer,
            format!("Unsupported context server `{context_server_id}`."),
        ));
    }

    let Some(binary_path) = settings.binary_path() else {
        return Err(diagnostic_with_action(
            DiagnosticPhase::Configuration,
            DiagnosticCode::BinaryPathNotConfigured,
            "`binary_path` is not configured.",
        ));
    };

    let path = Path::new(binary_path);
    if path.file_name().and_then(|name| name.to_str()) != Some(TRANSLATOR_MCP_BINARY) {
        return Err(diagnostic_for_artifact_status(
            PreparedArtifactStatus::IncompatibleCheckout,
        ));
    }

    let status = artifact_status(path);
    if status != PreparedArtifactStatus::Usable {
        return Err(diagnostic_for_artifact_status(status));
    }

    let mut env = settings.provider_env();
    if let Some(value) = settings.rust_log() {
        env.push(("RUST_LOG".to_string(), value.to_string()));
    }

    Ok(LaunchProfile {
        command: binary_path.to_string(),
        args: Vec::new(),
        env,
    })
}

/// Build the controlled Zed language-server command for the direct workflow.
pub fn build_lsp_launch_profile(
    language_server_id: &str,
    settings: &LaunchSettings,
) -> Result<LaunchProfile, DiagnosticEvent> {
    if language_server_id != DIRECT_LSP_ID {
        return Err(direct_diagnostic_with_action(
            DiagnosticPhase::Configuration,
            DiagnosticCode::UnsupportedContextServer,
            "Unsupported language server.",
        ));
    }

    let Some(binary_path) = settings.binary_path() else {
        return Err(direct_diagnostic_with_action(
            DiagnosticPhase::Configuration,
            DiagnosticCode::BinaryPathNotConfigured,
            "The direct translator binary is not configured.",
        ));
    };
    let path = Path::new(binary_path);
    if path.file_name().and_then(|name| name.to_str()) != Some(TRANSLATOR_LSP_BINARY) {
        return Err(direct_diagnostic_with_action(
            DiagnosticPhase::ArtifactValidation,
            DiagnosticCode::BinaryStaleOrIncompatible,
            "The configured direct translator artifact is incompatible.",
        ));
    }
    let status = artifact_status(path);
    if status != PreparedArtifactStatus::Usable {
        return Err(diagnostic_for_direct_artifact_status(status));
    }

    let mut env = settings.provider_env();
    if let Some(value) = settings.rust_log() {
        env.push(("RUST_LOG".to_string(), value.to_string()));
    }
    Ok(LaunchProfile {
        command: binary_path.to_string(),
        args: Vec::new(),
        env,
    })
}

fn diagnostic_for_direct_artifact_status(status: PreparedArtifactStatus) -> DiagnosticEvent {
    match status {
        PreparedArtifactStatus::Missing => direct_diagnostic_with_action(
            DiagnosticPhase::ArtifactValidation,
            DiagnosticCode::BinaryNotFound,
            "The configured direct translator artifact was not found.",
        ),
        PreparedArtifactStatus::NotExecutable => direct_diagnostic_with_action(
            DiagnosticPhase::ArtifactValidation,
            DiagnosticCode::BinaryNotExecutable,
            "The configured direct translator artifact is not executable.",
        ),
        PreparedArtifactStatus::NotFile
        | PreparedArtifactStatus::Stale
        | PreparedArtifactStatus::IncompatibleCheckout
        | PreparedArtifactStatus::FailedOnStart => direct_diagnostic_with_action(
            DiagnosticPhase::ArtifactValidation,
            DiagnosticCode::BinaryStaleOrIncompatible,
            "The configured direct translator artifact is stale or incompatible.",
        ),
        PreparedArtifactStatus::Usable => direct_diagnostic_with_action(
            DiagnosticPhase::ArtifactValidation,
            DiagnosticCode::InternalExtensionError,
            "No direct artifact failure was present.",
        ),
    }
}

/// Validate local artifact state from the filesystem.
///
/// Zed runs extensions as WASM. In that runtime, probing arbitrary host paths
/// with `std::fs::metadata` can report a missing file even when Zed can spawn
/// the configured command. Host-side preflight commands can also block the Zed
/// configuration modal. Native tests still exercise the stricter filesystem
/// checks; the WASM extension only validates command shape and lets Zed perform
/// the actual process launch.
#[cfg(target_arch = "wasm32")]
pub fn artifact_status(_path: &Path) -> PreparedArtifactStatus {
    PreparedArtifactStatus::Usable
}

/// Validate local artifact state from the filesystem.
#[cfg(not(target_arch = "wasm32"))]
pub fn artifact_status(path: &Path) -> PreparedArtifactStatus {
    let Ok(metadata) = path.metadata() else {
        return PreparedArtifactStatus::Missing;
    };

    if !metadata.is_file() {
        return PreparedArtifactStatus::NotFile;
    }

    if !is_executable(&metadata) {
        return PreparedArtifactStatus::NotExecutable;
    }

    PreparedArtifactStatus::Usable
}

/// Convert an artifact status to a stable redacted diagnostic.
pub fn diagnostic_for_artifact_status(status: PreparedArtifactStatus) -> DiagnosticEvent {
    match status {
        PreparedArtifactStatus::Missing => diagnostic_with_action(
            DiagnosticPhase::ArtifactValidation,
            DiagnosticCode::BinaryNotFound,
            "The configured translator-mcp artifact was not found.",
        ),
        PreparedArtifactStatus::NotFile => diagnostic_with_action(
            DiagnosticPhase::ArtifactValidation,
            DiagnosticCode::BinaryStaleOrIncompatible,
            "The configured translator-mcp artifact is not a file.",
        ),
        PreparedArtifactStatus::NotExecutable => diagnostic_with_action(
            DiagnosticPhase::ArtifactValidation,
            DiagnosticCode::BinaryNotExecutable,
            "The configured translator-mcp artifact is not executable.",
        ),
        PreparedArtifactStatus::Stale
        | PreparedArtifactStatus::IncompatibleCheckout
        | PreparedArtifactStatus::FailedOnStart => diagnostic_with_action(
            DiagnosticPhase::ArtifactValidation,
            DiagnosticCode::BinaryStaleOrIncompatible,
            "The configured translator-mcp artifact is stale or incompatible.",
        ),
        PreparedArtifactStatus::Usable => diagnostic_with_action(
            DiagnosticPhase::ArtifactValidation,
            DiagnosticCode::InternalExtensionError,
            "No artifact failure was present.",
        ),
    }
}

#[cfg(all(not(target_arch = "wasm32"), unix))]
fn is_executable(metadata: &std::fs::Metadata) -> bool {
    metadata.permissions().mode() & 0o111 != 0
}

#[cfg(all(not(target_arch = "wasm32"), not(unix)))]
fn is_executable(metadata: &std::fs::Metadata) -> bool {
    metadata.is_file()
}
