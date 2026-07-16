use crate::{ErrorCode, TranslateFailure};

pub const ENV_PROVIDER: &str = "TRANSLATOR_PROVIDER";
pub const ENV_PROVIDER_URL: &str = "TRANSLATOR_PROVIDER_URL";
pub const ENV_PROVIDER_API_KEY_ENV: &str = "TRANSLATOR_PROVIDER_API_KEY_ENV";
pub const ENV_ALLOW_REMOTE_PROVIDER: &str = "TRANSLATOR_ALLOW_REMOTE_PROVIDER";
pub const LIBRETRANSLATE_OPERATIONAL_URL: &str = "http://127.0.0.1:5000";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderMode {
    Mock,
    EmbeddedLocal,
    LibreTranslate,
    AzureTranslator,
}

impl ProviderMode {
    fn parse(value: Option<&str>) -> Result<Self, TranslateFailure> {
        match value.map(str::trim) {
            None => Ok(Self::Mock),
            Some("mock") => Ok(Self::Mock),
            Some("embedded_local") => Ok(Self::EmbeddedLocal),
            Some("libretranslate") => Ok(Self::LibreTranslate),
            Some("azure_translator") => Ok(Self::AzureTranslator),
            Some(_) => Err(provider_not_configured("Unsupported provider mode.")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderLocality {
    Local,
    NonLocal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderTarget {
    url: String,
    locality: ProviderLocality,
    allow_remote: bool,
}

impl ProviderTarget {
    pub fn parse(url: &str, allow_remote: bool) -> Result<Self, TranslateFailure> {
        let trimmed = url.trim().trim_end_matches('/');
        if trimmed.is_empty() {
            return Err(provider_not_configured("Provider URL is required."));
        }

        let after_scheme = trimmed
            .strip_prefix("http://")
            .or_else(|| trimmed.strip_prefix("https://"))
            .ok_or_else(|| provider_not_configured("Provider URL scheme is unsupported."))?;
        let authority = after_scheme
            .split(['/', '?', '#'])
            .next()
            .unwrap_or_default();
        if authority.is_empty() || authority.contains('@') {
            return Err(provider_not_configured("Provider URL target is unsafe."));
        }

        let host = host_from_authority(authority)
            .ok_or_else(|| provider_not_configured("Provider URL host is missing."))?;
        let locality = if is_loopback_host(host) {
            ProviderLocality::Local
        } else {
            ProviderLocality::NonLocal
        };

        Ok(Self {
            url: trimmed.to_string(),
            locality,
            allow_remote,
        })
    }

    pub fn base_url(&self) -> &str {
        &self.url
    }

    pub const fn locality(&self) -> ProviderLocality {
        self.locality
    }

    pub const fn allow_remote(&self) -> bool {
        self.allow_remote
    }

    pub fn translate_endpoint(&self) -> String {
        format!("{}/translate", self.url)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderConfiguration {
    pub mode: ProviderMode,
    pub target: Option<ProviderTarget>,
    pub api_key_env: Option<String>,
}

impl ProviderConfiguration {
    pub fn from_env() -> Result<Self, TranslateFailure> {
        let provider = std::env::var(ENV_PROVIDER).ok();
        let url = std::env::var(ENV_PROVIDER_URL).ok();
        let api_key_env = std::env::var(ENV_PROVIDER_API_KEY_ENV).ok();
        let allow_remote = std::env::var(ENV_ALLOW_REMOTE_PROVIDER).ok();

        Self::from_values(
            provider.as_deref(),
            url.as_deref(),
            api_key_env.as_deref(),
            allow_remote.as_deref(),
        )
    }

    pub fn from_values(
        provider: Option<&str>,
        url: Option<&str>,
        api_key_env: Option<&str>,
        allow_remote: Option<&str>,
    ) -> Result<Self, TranslateFailure> {
        let mode = ProviderMode::parse(provider)?;
        let (target, api_key_env) = match mode {
            ProviderMode::Mock => {
                reject_present(url, "Provider URL is not valid for mock mode.")?;
                reject_present(
                    api_key_env,
                    "Provider API key reference is not valid for mock mode.",
                )?;
                if parse_remote_allowance(allow_remote)? {
                    return Err(provider_not_configured(
                        "Remote allowance is not valid for mock mode.",
                    ));
                }
                (None, None)
            }
            ProviderMode::EmbeddedLocal => {
                reject_present(url, "Provider URL is not valid for embedded local mode.")?;
                reject_present(
                    api_key_env,
                    "Provider API key reference is not valid for embedded local mode.",
                )?;
                if allow_remote.is_some() {
                    return Err(provider_not_configured(
                        "Remote allowance is not valid for embedded local mode.",
                    ));
                }
                (None, None)
            }
            ProviderMode::LibreTranslate => {
                let url =
                    url.ok_or_else(|| provider_not_configured("Provider URL is required."))?;
                if url != LIBRETRANSLATE_OPERATIONAL_URL {
                    return Err(provider_not_configured(
                        "The local provider target is not supported.",
                    ));
                }
                reject_present(
                    api_key_env,
                    "Provider API key reference is not valid for local mode.",
                )?;
                if parse_remote_allowance(allow_remote)? {
                    return Err(provider_not_configured(
                        "Remote allowance is not valid for local mode.",
                    ));
                }
                (Some(ProviderTarget::parse(url, false)?), None)
            }
            ProviderMode::AzureTranslator => {
                reject_present(url, "Provider URL overrides are not supported.")?;
                let api_key_env = parse_required_api_key_env(api_key_env)?;
                if allow_remote.map(str::trim) != Some("true") {
                    return Err(provider_not_configured(
                        "Remote allowance must be enabled for this provider.",
                    ));
                }
                (None, Some(api_key_env))
            }
        };

        Ok(Self {
            mode,
            target,
            api_key_env,
        })
    }

    pub const fn uses_inherited_proxy_environment(&self) -> bool {
        false
    }
}

pub(crate) fn provider_not_configured(message: impl Into<String>) -> TranslateFailure {
    TranslateFailure::new(ErrorCode::ProviderNotConfigured, message)
}

fn parse_remote_allowance(value: Option<&str>) -> Result<bool, TranslateFailure> {
    match value.map(str::trim) {
        None | Some("false") => Ok(false),
        Some("true") => Ok(true),
        Some(_) => Err(provider_not_configured(
            "Provider remote allowance has an invalid value.",
        )),
    }
}

fn parse_required_api_key_env(value: Option<&str>) -> Result<String, TranslateFailure> {
    let trimmed = value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| provider_not_configured("Provider API key reference is required."))?;

    let mut chars = trimmed.chars();
    let Some(first) = chars.next() else {
        return Err(provider_not_configured(
            "Provider API key environment reference is invalid.",
        ));
    };
    if !(first == '_' || first.is_ascii_alphabetic())
        || !chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
    {
        return Err(provider_not_configured(
            "Provider API key environment reference is invalid.",
        ));
    }

    Ok(trimmed.to_string())
}

fn reject_present(value: Option<&str>, message: &'static str) -> Result<(), TranslateFailure> {
    if value.is_some() {
        return Err(provider_not_configured(message));
    }
    Ok(())
}

fn host_from_authority(authority: &str) -> Option<&str> {
    if let Some(rest) = authority.strip_prefix('[') {
        return rest.split_once(']').map(|(host, _)| host);
    }

    authority.split(':').next().filter(|host| !host.is_empty())
}

fn is_loopback_host(host: &str) -> bool {
    host.eq_ignore_ascii_case("localhost") || host == "127.0.0.1" || host == "::1"
}
