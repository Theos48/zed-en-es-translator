use std::collections::HashSet;
use std::path::Path;

use zed_extension_api::serde_json::Value;

use crate::diagnostics::{
    diagnostic_with_action, is_safe_rust_log, DiagnosticCode, DiagnosticEvent, DiagnosticPhase,
};

/// The only nested context-server setting accepted by this feature.
pub const ALLOWED_SETTING_NAMES: &[&str] = &["binary_path", "provider"];

/// Configuration keys that remain out of scope for the local wrapper.
pub const FORBIDDEN_SETTING_NAMES: &[&str] = &[
    "api_key",
    "base_url",
    "remote",
    "remote_confirmation",
    "headers",
    "extra_env",
    "extra_args",
];

const ENV_PROVIDER: &str = "TRANSLATOR_PROVIDER";
const ENV_PROVIDER_URL: &str = "TRANSLATOR_PROVIDER_URL";
const ENV_PROVIDER_API_KEY_ENV: &str = "TRANSLATOR_PROVIDER_API_KEY_ENV";
const ENV_ALLOW_REMOTE_PROVIDER: &str = "TRANSLATOR_ALLOW_REMOTE_PROVIDER";
const CONTROLLED_PROVIDER_ENV_NAMES: &[&str] = &[
    ENV_PROVIDER,
    ENV_PROVIDER_URL,
    ENV_PROVIDER_API_KEY_ENV,
    ENV_ALLOW_REMOTE_PROVIDER,
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
    provider: ProviderSettings,
    provider_configured: bool,
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

        let mut provider_environment = Vec::new();
        for (key, value) in command.env {
            if key == "RUST_LOG" && is_safe_rust_log(&value) {
                launch_settings.rust_log = Some(value);
            } else if CONTROLLED_PROVIDER_ENV_NAMES.contains(&key.as_str()) {
                provider_environment.push((key, value));
            } else {
                return Err(unsafe_setting("command.env"));
            }
        }
        if !provider_environment.is_empty() {
            launch_settings.provider = parse_provider_environment(provider_environment)?;
            launch_settings.provider_configured = true;
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

    /// Controlled provider environment entries passed to the selected server.
    pub fn provider_env(&self) -> Vec<(String, String)> {
        self.provider.env()
    }
}

/// Controlled provider settings accepted from Zed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderSettings {
    mode: String,
    url: Option<String>,
    api_key_env: Option<String>,
    allow_remote: bool,
}

impl Default for ProviderSettings {
    fn default() -> Self {
        Self {
            mode: "mock".to_string(),
            url: None,
            api_key_env: None,
            allow_remote: false,
        }
    }
}

impl ProviderSettings {
    fn env(&self) -> Vec<(String, String)> {
        if self.mode == "mock" {
            return Vec::new();
        }

        let mut env = vec![
            ("TRANSLATOR_PROVIDER".to_string(), self.mode.clone()),
            (
                "TRANSLATOR_PROVIDER_URL".to_string(),
                self.url.clone().unwrap_or_default(),
            ),
        ];
        if let Some(api_key_env) = &self.api_key_env {
            env.push((
                "TRANSLATOR_PROVIDER_API_KEY_ENV".to_string(),
                api_key_env.clone(),
            ));
        }
        env.push((
            "TRANSLATOR_ALLOW_REMOTE_PROVIDER".to_string(),
            self.allow_remote.to_string(),
        ));
        env
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
                return Err(invalid_binary_path_type());
            };
            set_binary_path(launch_settings, path.to_string(), "binary_path")?;
        } else if key == "provider" {
            if launch_settings.provider_configured {
                return Err(conflicting_provider_configuration());
            }
            launch_settings.provider = parse_provider_settings(value)?;
            launch_settings.provider_configured = true;
        }
    }

    Ok(())
}

fn parse_provider_settings(value: &Value) -> Result<ProviderSettings, DiagnosticEvent> {
    let Some(object) = value.as_object() else {
        return Err(unsafe_setting("provider"));
    };

    let mut provider = ProviderSettings::default();
    for (key, value) in object {
        match key.as_str() {
            "mode" => {
                let Some(mode) = value.as_str() else {
                    return Err(unsafe_setting("provider.mode"));
                };
                provider.mode = parse_provider_mode(mode)?;
            }
            "url" => {
                let Some(url) = value.as_str() else {
                    return Err(unsafe_setting("provider.url"));
                };
                provider.url = parse_provider_url(url)?;
            }
            "api_key_env" => {
                let Some(api_key_env) = value.as_str() else {
                    return Err(unsafe_setting("provider.api_key_env"));
                };
                provider.api_key_env = parse_api_key_env(api_key_env)?;
            }
            "allow_remote" => {
                let Some(allow_remote) = value.as_bool() else {
                    return Err(unsafe_setting("provider.allow_remote"));
                };
                provider.allow_remote = allow_remote;
            }
            _ => return Err(unsafe_setting(key)),
        }
    }

    validate_provider_settings(&provider)?;
    Ok(provider)
}

fn parse_provider_environment(
    entries: Vec<(String, String)>,
) -> Result<ProviderSettings, DiagnosticEvent> {
    let mut provider = ProviderSettings::default();
    let mut seen = HashSet::new();

    for (key, value) in entries {
        if !seen.insert(key.clone()) {
            return Err(unsafe_setting("command.env"));
        }
        match key.as_str() {
            ENV_PROVIDER => provider.mode = parse_provider_mode(&value)?,
            ENV_PROVIDER_URL => provider.url = parse_provider_url(&value)?,
            ENV_PROVIDER_API_KEY_ENV => provider.api_key_env = parse_api_key_env(&value)?,
            ENV_ALLOW_REMOTE_PROVIDER => {
                provider.allow_remote = parse_allow_remote_environment(&value)?;
            }
            _ => return Err(unsafe_setting("command.env")),
        }
    }

    if !seen.contains(ENV_PROVIDER) {
        return Err(unsafe_setting("command.env.TRANSLATOR_PROVIDER"));
    }
    validate_provider_settings(&provider)?;
    Ok(provider)
}

fn validate_provider_settings(provider: &ProviderSettings) -> Result<(), DiagnosticEvent> {
    if provider.mode == "libretranslate" && provider.url.is_none() {
        return Err(unsafe_setting("provider.url"));
    }
    if let Some(url) = &provider.url {
        if is_non_local_url(url) && !provider.allow_remote {
            return Err(unsafe_setting("provider.allow_remote"));
        }
    }

    Ok(())
}

fn parse_provider_mode(mode: &str) -> Result<String, DiagnosticEvent> {
    match mode.trim() {
        "" | "mock" => Ok("mock".to_string()),
        "libretranslate" => Ok("libretranslate".to_string()),
        _ => Err(unsafe_setting("provider.mode")),
    }
}

fn parse_provider_url(url: &str) -> Result<Option<String>, DiagnosticEvent> {
    let trimmed = url.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        return Ok(None);
    }
    let Some(after_scheme) = trimmed
        .strip_prefix("http://")
        .or_else(|| trimmed.strip_prefix("https://"))
    else {
        return Err(unsafe_setting("provider.url"));
    };
    let authority = after_scheme
        .split(['/', '?', '#'])
        .next()
        .unwrap_or_default();
    if authority.is_empty() || authority.contains('@') {
        return Err(unsafe_setting("provider.url"));
    }
    Ok(Some(trimmed.to_string()))
}

fn parse_api_key_env(value: &str) -> Result<Option<String>, DiagnosticEvent> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    let mut chars = trimmed.chars();
    let Some(first) = chars.next() else {
        return Ok(None);
    };
    if !(first == '_' || first.is_ascii_alphabetic())
        || !chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
    {
        return Err(unsafe_setting("provider.api_key_env"));
    }
    Ok(Some(trimmed.to_string()))
}

fn parse_allow_remote_environment(value: &str) -> Result<bool, DiagnosticEvent> {
    match value.trim() {
        "true" | "1" => Ok(true),
        "false" | "0" => Ok(false),
        _ => Err(unsafe_setting(
            "command.env.TRANSLATOR_ALLOW_REMOTE_PROVIDER",
        )),
    }
}

fn conflicting_provider_configuration() -> DiagnosticEvent {
    diagnostic_with_action(
        DiagnosticPhase::Configuration,
        DiagnosticCode::UnsafeLaunchConfiguration,
        "Rejected conflicting provider configuration from nested settings and binary environment.",
    )
}

fn is_non_local_url(url: &str) -> bool {
    let Some(after_scheme) = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
    else {
        return true;
    };
    let authority = after_scheme
        .split(['/', '?', '#'])
        .next()
        .unwrap_or_default();
    let host = if let Some(rest) = authority.strip_prefix('[') {
        rest.split_once(']').map(|(host, _)| host).unwrap_or("")
    } else {
        authority.split(':').next().unwrap_or("")
    };
    !(host.eq_ignore_ascii_case("localhost") || host == "127.0.0.1" || host == "::1")
}

fn set_binary_path(
    launch_settings: &mut LaunchSettings,
    path: String,
    source: &str,
) -> Result<(), DiagnosticEvent> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(empty_binary_path(source));
    }
    if !Path::new(trimmed).is_absolute() {
        return Err(relative_binary_path(source));
    }

    if let Some(existing) = launch_settings.binary_path() {
        if existing != trimmed {
            return Err(diagnostic_with_action(
                DiagnosticPhase::Configuration,
                DiagnosticCode::UnsafeLaunchConfiguration,
                "Rejected conflicting `binary_path` values between command settings and context server settings.",
            ));
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

fn empty_binary_path(source: &str) -> DiagnosticEvent {
    diagnostic_with_action(
        DiagnosticPhase::Configuration,
        DiagnosticCode::BinaryPathNotConfigured,
        format!("Rejected empty `{source}` value."),
    )
}

fn invalid_binary_path_type() -> DiagnosticEvent {
    diagnostic_with_action(
        DiagnosticPhase::Configuration,
        DiagnosticCode::UnsafeLaunchConfiguration,
        "Rejected `binary_path` because it must be a string.",
    )
}

fn relative_binary_path(source: &str) -> DiagnosticEvent {
    diagnostic_with_action(
        DiagnosticPhase::Configuration,
        DiagnosticCode::UnsafeLaunchConfiguration,
        format!("Rejected `{source}` because it must be an absolute path."),
    )
}
