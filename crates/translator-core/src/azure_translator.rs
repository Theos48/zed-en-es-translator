//! Fixed Azure Translator v3 adapter and its credential-isolating transport.

use std::fmt;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{
    ensure_provider_response_shape, provider_config::provider_not_configured, ErrorCode, Language,
    Provider, ProviderRequest, ProviderResponse, Tone, TranslateFailure, MAX_OUTPUT_BYTES,
    MAX_SEGMENTS, PROVIDER_TIMEOUT_MS,
};

/// Fixed reviewed Azure Translator v3 endpoint for English-to-Spanish requests.
pub const AZURE_TRANSLATOR_ENDPOINT: &str =
    "https://api.cognitive.microsofttranslator.com/translate?api-version=3.0&from=en&to=es";

const AZURE_RESPONSE_BODY_LIMIT_BYTES: u64 =
    (MAX_OUTPUT_BYTES + (MAX_SEGMENTS * 128) + 1024) as u64;

/// Credential-free transport boundary used by controlled tests.
///
/// Production owns its credential inside [`UreqAzureTransport`]; custom
/// transports receive only the minimized serialized body.
pub trait AzureTransport {
    /// Send one already validated, minimized request body.
    ///
    /// # Errors
    ///
    /// Returns a normalized transport category without raw provider detail.
    fn send(&self, body: &[u8]) -> Result<Vec<u8>, AzureTransportError>;
}

/// Redacted transport outcomes used by the Azure adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AzureTransportError {
    /// The configured provider time budget elapsed.
    Timeout,
    /// The provider returned HTTP 408.
    Http408,
    /// Hostname resolution failed.
    Dns,
    /// TLS verification or negotiation failed.
    Tls,
    /// The provider attempted a redirect.
    Redirect,
    /// The service rejected the request or returned a non-success status.
    Rejected,
    /// The bounded response body limit was exceeded.
    BodyTooLarge,
    /// Any other redacted transport failure.
    Failed,
}

#[derive(Clone)]
struct SecretValue(String);

#[derive(Clone)]
/// Production `ureq` transport whose credential-bearing fields stay private.
pub struct UreqAzureTransport {
    agent: ureq::Agent,
    subscription_key: SecretValue,
}

impl UreqAzureTransport {
    fn new(subscription_key: String) -> Self {
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_millis(PROVIDER_TIMEOUT_MS)))
            .proxy(None)
            .max_redirects(0)
            .https_only(true)
            .build()
            .into();
        Self {
            agent,
            subscription_key: SecretValue(subscription_key),
        }
    }
}

impl AzureTransport for UreqAzureTransport {
    fn send(&self, body: &[u8]) -> Result<Vec<u8>, AzureTransportError> {
        let mut response = self
            .agent
            .post(AZURE_TRANSLATOR_ENDPOINT)
            .header("Content-Type", "application/json; charset=UTF-8")
            .header("Ocp-Apim-Subscription-Key", &self.subscription_key.0)
            .send(body)
            .map_err(classify_ureq_error)?;

        if response.status().is_redirection() {
            return Err(AzureTransportError::Redirect);
        }
        if !response.status().is_success() {
            return Err(AzureTransportError::Rejected);
        }

        response
            .body_mut()
            .with_config()
            .limit(AZURE_RESPONSE_BODY_LIMIT_BYTES)
            .read_to_vec()
            .map_err(classify_ureq_error)
    }
}

/// Azure provider with a statically dispatched, credential-isolating transport.
#[derive(Clone)]
pub struct AzureTranslatorProvider<T = UreqAzureTransport> {
    transport: T,
}

impl AzureTranslatorProvider<UreqAzureTransport> {
    /// Resolve the referenced inherited environment variable and build the
    /// production HTTPS transport.
    ///
    /// # Errors
    ///
    /// Returns `PROVIDER_NOT_CONFIGURED` when the referenced value is absent
    /// or empty. The value is never copied into the error.
    pub fn from_env_reference(reference: &str) -> Result<Self, TranslateFailure> {
        let value = std::env::var(reference).map_err(|_| {
            provider_not_configured("Provider API key environment reference is not available.")
        })?;
        if value.trim().is_empty() {
            return Err(provider_not_configured(
                "Provider API key environment reference is empty.",
            ));
        }
        Ok(Self {
            transport: UreqAzureTransport::new(value),
        })
    }
}

impl<T> AzureTranslatorProvider<T> {
    /// Construct a provider around a credential-free controlled transport.
    pub const fn with_transport(transport: T) -> Self {
        Self { transport }
    }
}

impl<T> fmt::Debug for AzureTranslatorProvider<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AzureTranslatorProvider")
            .field("transport", &"redacted")
            .finish()
    }
}

impl<T: AzureTransport> Provider for AzureTranslatorProvider<T> {
    fn translate(&self, request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        validate_request_invariants(request)?;

        let elements = request
            .segments
            .iter()
            .map(|text| AzureRequestElement { text })
            .collect::<Vec<_>>();
        let body = serde_json::to_vec(&elements).map_err(|_| provider_failed())?;
        let response_body = self.transport.send(&body).map_err(map_transport_error)?;
        if response_body.len() as u64 > AZURE_RESPONSE_BODY_LIMIT_BYTES {
            return Err(provider_failed());
        }

        let response: Vec<AzureResponseElement> =
            serde_json::from_slice(&response_body).map_err(|_| provider_failed())?;
        if response.len() != request.segments.len() {
            return Err(provider_failed());
        }

        let translated_segments = response
            .into_iter()
            .map(AzureResponseElement::into_spanish_text)
            .collect::<Result<Vec<_>, _>>()?;
        let response = ProviderResponse {
            translated_segments,
        };
        ensure_provider_response_shape(request, &response)?;
        Ok(response)
    }
}

#[derive(Serialize)]
struct AzureRequestElement<'a> {
    #[serde(rename = "Text")]
    text: &'a str,
}

#[derive(Deserialize)]
struct AzureResponseElement {
    translations: Vec<AzureTranslation>,
}

impl AzureResponseElement {
    fn into_spanish_text(self) -> Result<String, TranslateFailure> {
        if self.translations.len() != 1 {
            return Err(provider_failed());
        }
        let translation = self
            .translations
            .into_iter()
            .next()
            .ok_or_else(provider_failed)?;
        if translation.text.trim().is_empty()
            || translation
                .to
                .as_deref()
                .is_some_and(|language| language != "es")
        {
            return Err(provider_failed());
        }
        Ok(translation.text)
    }
}

#[derive(Deserialize)]
struct AzureTranslation {
    text: String,
    to: Option<String>,
}

fn validate_request_invariants(request: &ProviderRequest) -> Result<(), TranslateFailure> {
    if request.source_language != Language::English || request.target_language != Language::Spanish
    {
        return Err(TranslateFailure::new(
            ErrorCode::UnsupportedLanguagePair,
            "Unsupported language pair.",
        ));
    }
    if request.tone != Tone::TechnicalNeutral || !request.preserve_formatting {
        return Err(TranslateFailure::invalid_input(
            "Unsupported provider request invariants.",
        ));
    }
    Ok(())
}

fn classify_ureq_error(error: ureq::Error) -> AzureTransportError {
    match error {
        ureq::Error::Timeout(_) => AzureTransportError::Timeout,
        ureq::Error::StatusCode(408) => AzureTransportError::Http408,
        ureq::Error::StatusCode(_) => AzureTransportError::Rejected,
        ureq::Error::HostNotFound => AzureTransportError::Dns,
        ureq::Error::Tls(_) => AzureTransportError::Tls,
        ureq::Error::RedirectFailed | ureq::Error::TooManyRedirects => {
            AzureTransportError::Redirect
        }
        ureq::Error::BodyExceedsLimit(_) => AzureTransportError::BodyTooLarge,
        _ => AzureTransportError::Failed,
    }
}

fn map_transport_error(error: AzureTransportError) -> TranslateFailure {
    match error {
        AzureTransportError::Timeout | AzureTransportError::Http408 => TranslateFailure::new(
            ErrorCode::ProviderTimeout,
            "Provider request exceeded the configured timeout.",
        ),
        AzureTransportError::Dns
        | AzureTransportError::Tls
        | AzureTransportError::Redirect
        | AzureTransportError::Rejected
        | AzureTransportError::BodyTooLarge
        | AzureTransportError::Failed => provider_failed(),
    }
}

fn provider_failed() -> TranslateFailure {
    TranslateFailure::new(ErrorCode::ProviderFailed, "Provider request failed.")
}
