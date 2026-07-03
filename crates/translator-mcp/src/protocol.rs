//! MCP protocol-facing request and result shapes.

use rmcp::model::{CallToolResult, ContentBlock};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use translator_core::{
    redact_failure, ErrorCode, TranslateFailure, TranslateSuccess, MAX_INPUT_BYTES,
};

/// Name of the direct text translation tool.
pub const TRANSLATE_TEXT_TOOL_NAME: &str = "translate_text";

/// Name of the workspace file translation tool.
pub const TRANSLATE_FILE_TOOL_NAME: &str = "translate_file";

/// MCP parameters for `translate_text`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TranslateTextParams {
    /// English text to translate.
    pub source_text: String,
    /// Optional source language. Only `en` is supported in this feature.
    pub source_language: Option<String>,
    /// Optional target language. Only `es` is supported in this feature.
    pub target_language: Option<String>,
    /// Optional tone. Only `technical_neutral` is supported in this feature.
    pub tone: Option<String>,
    /// Optional formatting flag. `false` is rejected in this feature.
    pub preserve_formatting: Option<bool>,
}

impl TranslateTextParams {
    /// Validate MCP-only parameters before calling the translation core.
    pub fn validate(&self) -> Result<(), TranslateFailure> {
        validate_language_pair(
            self.source_language.as_deref(),
            self.target_language.as_deref(),
        )?;
        validate_tone(self.tone.as_deref())?;
        validate_preserve_formatting(self.preserve_formatting)?;
        if self.source_text.len() > MAX_INPUT_BYTES {
            return Err(TranslateFailure::new(
                ErrorCode::FileTooLarge,
                "The input exceeds the configured size limit.",
            ));
        }
        Ok(())
    }
}

/// MCP parameters for `translate_file`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TranslateFileParams {
    /// Authorized workspace root supplied by the client context.
    pub workspace_root: String,
    /// Requested file path inside the authorized workspace.
    pub file_path: String,
    /// Optional source language. Only `en` is supported in this feature.
    pub source_language: Option<String>,
    /// Optional target language. Only `es` is supported in this feature.
    pub target_language: Option<String>,
    /// Optional tone. Only `technical_neutral` is supported in this feature.
    pub tone: Option<String>,
    /// Optional formatting flag. `false` is rejected in this feature.
    pub preserve_formatting: Option<bool>,
}

impl TranslateFileParams {
    /// Validate MCP-only parameters before delegating file policy to the core.
    pub fn validate(&self) -> Result<(), TranslateFailure> {
        validate_language_pair(
            self.source_language.as_deref(),
            self.target_language.as_deref(),
        )?;
        validate_tone(self.tone.as_deref())?;
        validate_preserve_formatting(self.preserve_formatting)?;
        if self.workspace_root.trim().is_empty() || self.file_path.trim().is_empty() {
            return Err(TranslateFailure::invalid_input(
                "Workspace root and file path are required.",
            ));
        }
        Ok(())
    }
}

/// JSON Schema for the `translate_text` tool input.
pub fn translate_text_input_schema() -> Value {
    json!({
        "type": "object",
        "additionalProperties": false,
        "required": ["source_text"],
        "properties": {
            "source_text": {
                "type": "string",
                "minLength": 1,
                "pattern": "\\S",
                "description": "English text to translate."
            },
            "source_language": {
                "type": "string",
                "const": "en",
                "default": "en"
            },
            "target_language": {
                "type": "string",
                "const": "es",
                "default": "es"
            },
            "tone": {
                "type": "string",
                "enum": ["technical_neutral"],
                "default": "technical_neutral"
            },
            "preserve_formatting": {
                "type": "boolean",
                "const": true,
                "default": true
            }
        }
    })
}

/// JSON Schema for the `translate_file` tool input.
pub fn translate_file_input_schema() -> Value {
    json!({
        "type": "object",
        "additionalProperties": false,
        "required": ["workspace_root", "file_path"],
        "properties": {
            "workspace_root": {
                "type": "string",
                "minLength": 1,
                "description": "Authorized workspace root supplied by the client context."
            },
            "file_path": {
                "type": "string",
                "minLength": 1,
                "description": "Requested path inside the authorized workspace."
            },
            "source_language": {
                "type": "string",
                "const": "en",
                "default": "en"
            },
            "target_language": {
                "type": "string",
                "const": "es",
                "default": "es"
            },
            "tone": {
                "type": "string",
                "enum": ["technical_neutral"],
                "default": "technical_neutral"
            },
            "preserve_formatting": {
                "type": "boolean",
                "const": true,
                "default": true
            }
        }
    })
}

/// Build a successful MCP tool result from a core translation success.
pub fn success_result(success: TranslateSuccess) -> CallToolResult {
    let translated_text = success.translated_text;
    let mut result = CallToolResult::success(vec![ContentBlock::text(translated_text.clone())]);
    result.structured_content = Some(json!({ "translated_text": translated_text }));
    result
}

/// Build an error MCP tool result from a redacted core translation failure.
pub fn error_result(failure: TranslateFailure) -> CallToolResult {
    let failure = redact_failure(failure);
    let code = failure.code.as_str();
    let message = failure.message;
    let mut result = CallToolResult::error(vec![ContentBlock::text(format!("{code}: {message}"))]);
    result.structured_content = Some(json!({
        "code": code,
        "message": message
    }));
    result
}

fn validate_language_pair(
    source_language: Option<&str>,
    target_language: Option<&str>,
) -> Result<(), TranslateFailure> {
    if source_language.is_some_and(|value| value != "en")
        || target_language.is_some_and(|value| value != "es")
    {
        return Err(TranslateFailure::new(
            ErrorCode::UnsupportedLanguagePair,
            "Unsupported language pair.",
        ));
    }
    Ok(())
}

fn validate_tone(tone: Option<&str>) -> Result<(), TranslateFailure> {
    if tone.is_some_and(|value| value != "technical_neutral") {
        return Err(TranslateFailure::invalid_input("Unsupported tone."));
    }
    Ok(())
}

fn validate_preserve_formatting(value: Option<bool>) -> Result<(), TranslateFailure> {
    if value == Some(false) {
        return Err(TranslateFailure::invalid_input(
            "Formatting preservation must be enabled.",
        ));
    }
    Ok(())
}
