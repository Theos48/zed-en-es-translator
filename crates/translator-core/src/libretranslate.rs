use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{
    provider_config::provider_not_configured, ErrorCode, Language, Provider, ProviderRequest,
    ProviderResponse, ProviderTarget, TranslateFailure, MAX_OUTPUT_BYTES, PROVIDER_TIMEOUT_MS,
};

const PROVIDER_RESPONSE_BODY_LIMIT_BYTES: u64 = MAX_OUTPUT_BYTES as u64 + 1024;

#[derive(Debug, Clone)]
pub struct LibreTranslateProvider {
    target: ProviderTarget,
    api_key_env: Option<String>,
    agent: ureq::Agent,
}

impl LibreTranslateProvider {
    pub fn new(target: ProviderTarget, api_key_env: Option<String>) -> Self {
        Self::with_timeout(
            target,
            api_key_env,
            Duration::from_millis(PROVIDER_TIMEOUT_MS),
        )
    }

    pub fn with_timeout(
        target: ProviderTarget,
        api_key_env: Option<String>,
        timeout: Duration,
    ) -> Self {
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .timeout_global(Some(timeout))
            .proxy(None)
            .build()
            .into();
        Self {
            target,
            api_key_env,
            agent,
        }
    }

    pub fn target(&self) -> &ProviderTarget {
        &self.target
    }
}

impl Provider for LibreTranslateProvider {
    fn translate(&self, request: &ProviderRequest) -> Result<ProviderResponse, TranslateFailure> {
        if request.source_language != Language::English
            || request.target_language != Language::Spanish
        {
            return Err(TranslateFailure::new(
                ErrorCode::UnsupportedLanguagePair,
                "Unsupported language pair.",
            ));
        }

        let translated_segments = self.translate_segments(&request.segments)?;

        Ok(ProviderResponse {
            translated_segments,
        })
    }
}

impl LibreTranslateProvider {
    fn translate_segments(&self, segments: &[String]) -> Result<Vec<String>, TranslateFailure> {
        let payload = LibreTranslateRequest {
            q: segments,
            source: "en",
            target: "es",
            format: "text",
            alternatives: 0,
            api_key: self.api_key_value()?,
        };

        let endpoint = self.target.translate_endpoint();
        let mut response = self
            .agent
            .post(&endpoint)
            .header("Content-Type", "application/json")
            .send_json(&payload)
            .map_err(map_ureq_error)?;
        let body = response
            .body_mut()
            .with_config()
            .limit(PROVIDER_RESPONSE_BODY_LIMIT_BYTES)
            .read_json::<LibreTranslateResponse>()
            .map_err(map_ureq_error)?;
        let translated_segments = body.translated_text.into_segments(segments.len())?;

        if translated_segments.len() != segments.len() {
            return Err(provider_failed(
                "Provider returned an invalid segment count.",
            ));
        }
        if translated_segments
            .iter()
            .any(|segment| segment.trim().is_empty())
        {
            return Err(provider_failed("Provider returned empty translated text."));
        }

        Ok(translated_segments)
    }

    fn api_key_value(&self) -> Result<Option<String>, TranslateFailure> {
        let Some(env_name) = &self.api_key_env else {
            return Ok(None);
        };
        let value = std::env::var(env_name).map_err(|_| {
            provider_not_configured("Provider API key environment reference is not available.")
        })?;
        if value.trim().is_empty() {
            return Err(provider_not_configured(
                "Provider API key environment reference is empty.",
            ));
        }
        Ok(Some(value))
    }
}

#[derive(Debug, Serialize)]
struct LibreTranslateRequest<'a> {
    q: &'a [String],
    source: &'static str,
    target: &'static str,
    format: &'static str,
    alternatives: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LibreTranslateResponse {
    #[serde(rename = "translatedText")]
    translated_text: TranslatedText,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum TranslatedText {
    Single(String),
    Multiple(Vec<String>),
}

impl TranslatedText {
    fn into_segments(self, expected_segments: usize) -> Result<Vec<String>, TranslateFailure> {
        match self {
            Self::Single(text) if expected_segments == 1 => Ok(vec![text]),
            Self::Single(_) => Err(provider_failed(
                "Provider returned an invalid segment count.",
            )),
            Self::Multiple(segments) => Ok(segments),
        }
    }
}

fn map_ureq_error(error: ureq::Error) -> TranslateFailure {
    match error {
        ureq::Error::Timeout(_) => TranslateFailure::new(
            ErrorCode::ProviderTimeout,
            "Provider request exceeded the configured timeout.",
        ),
        _ => provider_failed("Provider request failed."),
    }
}

fn provider_failed(message: impl Into<String>) -> TranslateFailure {
    TranslateFailure::new(ErrorCode::ProviderFailed, message)
}
