use std::fmt;

/// Phase where a redacted startup diagnostic was produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticPhase {
    /// User or extension configuration validation.
    Configuration,
    /// Local server artifact validation.
    ArtifactValidation,
    /// Process launch preparation.
    Launch,
    /// Runtime status after the server has been started.
    ServerRuntime,
}

impl DiagnosticPhase {
    /// Stable string used in logs and test assertions.
    pub const fn as_str(self) -> &'static str {
        match self {
            DiagnosticPhase::Configuration => "configuration",
            DiagnosticPhase::ArtifactValidation => "artifact_validation",
            DiagnosticPhase::Launch => "launch",
            DiagnosticPhase::ServerRuntime => "server_runtime",
        }
    }
}

/// Stable Zed wrapper startup diagnostic categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCode {
    /// The local `translator-mcp` path has not been configured.
    BinaryPathNotConfigured,
    /// The configured `translator-mcp` path does not exist.
    BinaryNotFound,
    /// The configured artifact exists but is not executable.
    BinaryNotExecutable,
    /// The configured artifact is stale or does not match this checkout.
    BinaryStaleOrIncompatible,
    /// Zed asked the extension for an unsupported context server.
    UnsupportedContextServer,
    /// Settings attempted to add provider, remote, env, or arg behavior.
    UnsafeLaunchConfiguration,
    /// An unexpected extension-side error occurred.
    InternalExtensionError,
}

impl DiagnosticCode {
    /// Stable string exposed to Zed and tests.
    pub const fn as_str(self) -> &'static str {
        match self {
            DiagnosticCode::BinaryPathNotConfigured => "BINARY_PATH_NOT_CONFIGURED",
            DiagnosticCode::BinaryNotFound => "BINARY_NOT_FOUND",
            DiagnosticCode::BinaryNotExecutable => "BINARY_NOT_EXECUTABLE",
            DiagnosticCode::BinaryStaleOrIncompatible => "BINARY_STALE_OR_INCOMPATIBLE",
            DiagnosticCode::UnsupportedContextServer => "UNSUPPORTED_CONTEXT_SERVER",
            DiagnosticCode::UnsafeLaunchConfiguration => "UNSAFE_LAUNCH_CONFIGURATION",
            DiagnosticCode::InternalExtensionError => "INTERNAL_EXTENSION_ERROR",
        }
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Redacted extension diagnostic safe for user-visible Zed errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticEvent {
    /// Startup phase where the event occurred.
    pub phase: DiagnosticPhase,
    /// Stable diagnostic category.
    pub code: DiagnosticCode,
    /// Redacted user-actionable message.
    pub message: String,
    /// Optional bounded duration for manual smoke validation.
    pub duration_ms: Option<u128>,
}

impl DiagnosticEvent {
    /// Create a diagnostic and redact sensitive text from the message.
    pub fn new(phase: DiagnosticPhase, code: DiagnosticCode, message: impl AsRef<str>) -> Self {
        Self {
            phase,
            code,
            message: redact_sensitive(message.as_ref()),
            duration_ms: None,
        }
    }

    /// Attach a duration in milliseconds without changing the stable code.
    pub const fn with_duration_ms(mut self, duration_ms: u128) -> Self {
        self.duration_ms = Some(duration_ms);
        self
    }

    /// Format as a single redacted user-facing line.
    pub fn to_user_message(&self) -> String {
        match self.duration_ms {
            Some(duration_ms) => format!(
                "{} phase={} duration_ms={}: {}",
                self.code,
                self.phase.as_str(),
                duration_ms,
                self.message
            ),
            None => format!(
                "{} phase={}: {}",
                self.code,
                self.phase.as_str(),
                self.message
            ),
        }
    }
}

impl fmt::Display for DiagnosticEvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_user_message())
    }
}

/// Standard corrective action for a diagnostic category.
pub const fn corrective_action(code: DiagnosticCode) -> &'static str {
    match code {
        DiagnosticCode::BinaryPathNotConfigured => {
            "Run `make zed-extension-prepare` and configure `binary_path` to the printed translator-mcp artifact."
        }
        DiagnosticCode::BinaryNotFound => {
            "Run `make zed-extension-prepare` again and configure `binary_path` to the printed translator-mcp artifact."
        }
        DiagnosticCode::BinaryNotExecutable => {
            "Rebuild the release artifact with `make zed-extension-prepare` so Zed can execute translator-mcp directly."
        }
        DiagnosticCode::BinaryStaleOrIncompatible => {
            "Rebuild the local artifact from this checkout with `make zed-extension-prepare`."
        }
        DiagnosticCode::UnsupportedContextServer => {
            "Use the `translator-en-es` context server declared by this extension."
        }
        DiagnosticCode::UnsafeLaunchConfiguration => {
            "Remove provider, remote, extra argument, or arbitrary environment settings from the Zed context server configuration."
        }
        DiagnosticCode::InternalExtensionError => {
            "Retry after rebuilding with `make zed-extension-prepare`; inspect project tests if the error persists."
        }
    }
}

/// Build a redacted diagnostic with the standard corrective action appended.
pub fn diagnostic_with_action(
    phase: DiagnosticPhase,
    code: DiagnosticCode,
    detail: impl AsRef<str>,
) -> DiagnosticEvent {
    DiagnosticEvent::new(
        phase,
        code,
        format!("{} {}", detail.as_ref(), corrective_action(code)),
    )
}

/// Redact source text, translations, secrets, env dumps, URLs, and full paths.
pub fn redact_sensitive(input: &str) -> String {
    let replaced = input
        .replace("Read the docs.", "[redacted-content]")
        .replace("Lee la documentacion.", "[redacted-content]");
    redact_words(&replaced)
}

fn redact_words(input: &str) -> String {
    let mut output = Vec::new();
    let mut redact_next = false;

    for raw_word in input.split_whitespace() {
        if redact_next {
            output.push("[redacted-secret]".to_string());
            redact_next = false;
            continue;
        }

        let lower = raw_word.to_ascii_lowercase();
        if lower == "bearer" || lower.ends_with(":bearer") || lower.ends_with("bearer") {
            output.push(raw_word.to_string());
            redact_next = true;
            continue;
        }

        if let Some(redacted) = redact_assignment(raw_word) {
            output.push(redacted);
        } else if is_url_like(raw_word) {
            output.push("[redacted-url]".to_string());
        } else if is_path_like(raw_word) {
            output.push("[redacted-path]".to_string());
        } else {
            output.push(raw_word.to_string());
        }
    }

    output.join(" ")
}

fn redact_assignment(word: &str) -> Option<String> {
    let (key, value) = word.split_once('=')?;
    let clean_key =
        key.trim_matches(|character: char| !character.is_ascii_alphanumeric() && character != '_');
    let clean_value = value.trim_matches(|character: char| {
        matches!(character, '"' | '\'' | ',' | ';' | ')' | ']' | '}')
    });

    if clean_key == "RUST_LOG" && is_safe_rust_log(clean_value) {
        return None;
    }

    let upper_key = clean_key.to_ascii_uppercase();
    let sensitive_key = upper_key.contains("TOKEN")
        || upper_key.contains("SECRET")
        || upper_key.contains("KEY")
        || upper_key.contains("PASSWORD")
        || upper_key.contains("AUTH")
        || upper_key == "PATH"
        || upper_key == "HOME"
        || upper_key == "PWD";

    if sensitive_key || is_path_like(clean_value) || is_url_like(clean_value) {
        Some(format!("{clean_key}=[redacted]"))
    } else {
        None
    }
}

fn is_url_like(value: &str) -> bool {
    let trimmed = value.trim_matches(|character: char| {
        matches!(character, '"' | '\'' | ',' | ';' | ')' | ']' | '}')
    });

    trimmed.starts_with("http://") || trimmed.starts_with("https://")
}

fn is_path_like(value: &str) -> bool {
    let trimmed = value.trim_matches(|character: char| {
        matches!(character, '"' | '\'' | ',' | ';' | ':' | ')' | ']' | '}')
    });

    trimmed.starts_with('/')
        || trimmed.starts_with("~/")
        || trimmed.contains("/home/")
        || trimmed.contains("/workspace/")
        || trimmed.contains("\\Users\\")
}

fn is_safe_rust_log(value: &str) -> bool {
    matches!(value, "error" | "warn" | "info" | "debug" | "trace")
}
