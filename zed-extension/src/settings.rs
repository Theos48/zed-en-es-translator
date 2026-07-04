use std::collections::HashSet;

use zed_extension_api::serde_json::Value;

use crate::diagnostics::{
    diagnostic_with_action, DiagnosticCode, DiagnosticEvent, DiagnosticPhase,
};

/// The only nested context-server setting accepted by this feature.
pub const ALLOWED_SETTING_NAMES: &[&str] = &["binary_path"];

/// Configuration keys that remain out of scope for the local wrapper.
pub const FORBIDDEN_SETTING_NAMES: &[&str] = &[
    "provider",
    "api_key",
    "base_url",
    "remote",
    "remote_confirmation",
    "headers",
    "extra_env",
    "extra_args",
];

/// Command settings normalized from Zed before validation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CommandSettingsInput {
    /// Optional direct binary path supplied through Zed command settings.
    pub path: Option<String>,
    /// Extra arguments are rejected for this feature.
    pub arguments: Vec<String>,
    /// Environment entries are rejected except the explicit allowlist.
    pub env: Vec<(String, String)>,
}

impl CommandSettingsInput {
    /// Build normalized command settings for tests and the Zed adapter.
    pub fn new(path: Option<String>, arguments: Vec<String>, env: Vec<(String, String)>) -> Self {
        Self {
            path,
            arguments,
            env,
        }
    }
}

/// Validated settings needed to construct the launch profile.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LaunchSettings {
    binary_path: Option<String>,
    rust_log: Option<String>,
}

impl LaunchSettings {
    /// Validate nested context-server settings without command overrides.
    pub fn from_json_value(settings: Option<&Value>) -> Result<Self, DiagnosticEvent> {
        Self::from_parts(settings, CommandSettingsInput::default())
    }

    /// Validate nested settings and normalized command settings.
    pub fn from_parts(
        settings: Option<&Value>,
        command: CommandSettingsInput,
    ) -> Result<Self, DiagnosticEvent> {
        let mut launch_settings = Self::default();

        if let Some(path) = command.path {
            set_binary_path(&mut launch_settings, path, "command.path")?;
        }

        if !command.arguments.is_empty() {
            return Err(unsafe_setting("command.arguments"));
        }

        for (key, value) in command.env {
            if key == "RUST_LOG" && is_safe_rust_log(&value) {
                launch_settings.rust_log = Some(value);
            } else {
                return Err(unsafe_setting("command.env"));
            }
        }

        if let Some(settings_value) = settings {
            parse_nested_settings(settings_value, &mut launch_settings)?;
        }

        Ok(launch_settings)
    }

    /// Configured local server artifact path, if present.
    pub fn binary_path(&self) -> Option<&str> {
        self.binary_path.as_deref()
    }

    /// Optional explicitly allowlisted `RUST_LOG` value.
    pub fn rust_log(&self) -> Option<&str> {
        self.rust_log.as_deref()
    }
}

fn parse_nested_settings(
    settings: &Value,
    launch_settings: &mut LaunchSettings,
) -> Result<(), DiagnosticEvent> {
    let Some(object) = settings.as_object() else {
        return Err(unsafe_setting("settings"));
    };

    let allowed: HashSet<&str> = ALLOWED_SETTING_NAMES.iter().copied().collect();
    for (key, value) in object {
        if !allowed.contains(key.as_str()) {
            return Err(unsafe_setting(key));
        }

        if key == "binary_path" {
            let Some(path) = value.as_str() else {
                return Err(unsafe_setting("binary_path"));
            };
            set_binary_path(launch_settings, path.to_string(), "binary_path")?;
        }
    }

    Ok(())
}

fn set_binary_path(
    launch_settings: &mut LaunchSettings,
    path: String,
    source: &str,
) -> Result<(), DiagnosticEvent> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(unsafe_setting(source));
    }

    if let Some(existing) = launch_settings.binary_path() {
        if existing != trimmed {
            return Err(unsafe_setting("binary_path"));
        }
    }

    launch_settings.binary_path = Some(trimmed.to_string());
    Ok(())
}

fn unsafe_setting(name: &str) -> DiagnosticEvent {
    diagnostic_with_action(
        DiagnosticPhase::Configuration,
        DiagnosticCode::UnsafeLaunchConfiguration,
        format!("Rejected unsupported Zed context server setting `{name}`."),
    )
}

fn is_safe_rust_log(value: &str) -> bool {
    matches!(value, "error" | "warn" | "info" | "debug" | "trace")
}
