use std::collections::BTreeMap;

use crate::errors::ErrorCode;
use crate::limits::{MAX_INPUT_BYTES, MAX_OUTPUT_BYTES, MAX_SEGMENTS, MAX_SEGMENT_BYTES};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Spanish,
}

impl Language {
    pub const fn as_str(self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Spanish => "es",
        }
    }

    fn parse_source(value: &str) -> Result<Self, TranslateFailure> {
        match value {
            "en" => Ok(Language::English),
            _ => Err(TranslateFailure::new(
                ErrorCode::UnsupportedLanguagePair,
                "Unsupported source language.",
            )),
        }
    }

    fn parse_target(value: &str) -> Result<Self, TranslateFailure> {
        match value {
            "es" => Ok(Language::Spanish),
            _ => Err(TranslateFailure::new(
                ErrorCode::UnsupportedLanguagePair,
                "Unsupported target language.",
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    TechnicalNeutral,
}

impl Tone {
    pub const fn as_str(self) -> &'static str {
        match self {
            Tone::TechnicalNeutral => "technical_neutral",
        }
    }

    fn parse(value: &str) -> Result<Self, TranslateFailure> {
        match value {
            "technical_neutral" => Ok(Tone::TechnicalNeutral),
            _ => Err(TranslateFailure::invalid_input("Unsupported tone.")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputKind {
    Text,
    Markdown,
}

impl InputKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            InputKind::Text => "text",
            InputKind::Markdown => "markdown",
        }
    }

    fn parse(value: &str) -> Result<Self, TranslateFailure> {
        match value {
            "text" => Ok(InputKind::Text),
            "markdown" => Ok(InputKind::Markdown),
            _ => Err(TranslateFailure::invalid_input("Unsupported input kind.")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranslateRequest {
    pub source_text: Option<String>,
    pub source_language: Language,
    pub target_language: Language,
    pub tone: Tone,
    pub preserve_formatting: bool,
    pub input_kind: InputKind,
    pub file_path: Option<String>,
    pub workspace_root: Option<String>,
    pub remote_confirmed: bool,
}

impl TranslateRequest {
    pub fn direct_text(source_text: impl Into<String>) -> Self {
        Self {
            source_text: Some(source_text.into()),
            source_language: Language::English,
            target_language: Language::Spanish,
            tone: Tone::TechnicalNeutral,
            preserve_formatting: true,
            input_kind: InputKind::Text,
            file_path: None,
            workspace_root: None,
            remote_confirmed: false,
        }
    }

    pub fn file(
        file_path: impl Into<String>,
        workspace_root: impl Into<String>,
        input_kind: InputKind,
    ) -> Self {
        Self {
            source_text: None,
            source_language: Language::English,
            target_language: Language::Spanish,
            tone: Tone::TechnicalNeutral,
            preserve_formatting: true,
            input_kind,
            file_path: Some(file_path.into()),
            workspace_root: Some(workspace_root.into()),
            remote_confirmed: false,
        }
    }

    pub fn from_json(input: &str) -> Result<Self, TranslateFailure> {
        let object = parse_json_object(input)?;
        reject_unknown_fields(&object)?;

        let source_language = Language::parse_source(required_string(&object, "source_language")?)?;
        let target_language = Language::parse_target(required_string(&object, "target_language")?)?;
        let tone = Tone::parse(required_string(&object, "tone")?)?;
        let preserve_formatting = required_bool(&object, "preserve_formatting")?;
        if !preserve_formatting {
            return Err(TranslateFailure::invalid_input(
                "Formatting preservation must be enabled.",
            ));
        }
        let input_kind = InputKind::parse(required_string(&object, "input_kind")?)?;

        let source_text = optional_string(&object, "source_text")?;
        let file_path = optional_string(&object, "file_path")?;
        let workspace_root = optional_string(&object, "workspace_root")?;
        let remote_confirmed = optional_bool(&object, "remote_confirmed")?.unwrap_or(false);

        match (&source_text, &file_path, &workspace_root) {
            (Some(text), None, None) => {
                validate_input_size(text)?;
            }
            (None, Some(_), Some(_)) => {}
            _ => {
                return Err(TranslateFailure::invalid_input(
                    "Request must contain either source_text or file_path with workspace_root.",
                ));
            }
        }

        Ok(Self {
            source_text,
            source_language,
            target_language,
            tone,
            preserve_formatting,
            input_kind,
            file_path,
            workspace_root,
            remote_confirmed,
        })
    }

    pub fn to_json(&self) -> String {
        let mut json = String::from("{");
        let mut first = true;

        if let Some(source_text) = &self.source_text {
            push_string_field(&mut json, &mut first, "source_text", source_text);
        }
        push_string_field(
            &mut json,
            &mut first,
            "source_language",
            self.source_language.as_str(),
        );
        push_string_field(
            &mut json,
            &mut first,
            "target_language",
            self.target_language.as_str(),
        );
        push_string_field(&mut json, &mut first, "tone", self.tone.as_str());
        push_bool_field(
            &mut json,
            &mut first,
            "preserve_formatting",
            self.preserve_formatting,
        );
        push_string_field(
            &mut json,
            &mut first,
            "input_kind",
            self.input_kind.as_str(),
        );
        if let Some(file_path) = &self.file_path {
            push_string_field(&mut json, &mut first, "file_path", file_path);
        }
        if let Some(workspace_root) = &self.workspace_root {
            push_string_field(&mut json, &mut first, "workspace_root", workspace_root);
        }
        if self.remote_confirmed {
            push_bool_field(&mut json, &mut first, "remote_confirmed", true);
        }

        json.push('}');
        json
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranslateSuccess {
    pub translated_text: String,
}

impl TranslateSuccess {
    pub fn new(translated_text: impl Into<String>) -> Result<Self, TranslateFailure> {
        let translated_text = translated_text.into();
        if translated_text.len() > MAX_OUTPUT_BYTES {
            return Err(TranslateFailure::new(
                ErrorCode::ProviderFailed,
                "Provider output exceeds the configured size limit.",
            ));
        }
        Ok(Self { translated_text })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranslateFailure {
    pub code: ErrorCode,
    pub message: String,
}

impl TranslateFailure {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::InvalidInput, message)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranslateResult {
    Success(TranslateSuccess),
    Failure(TranslateFailure),
}

impl TranslateResult {
    pub fn to_json(&self) -> String {
        match self {
            TranslateResult::Success(success) => {
                format!(
                    r#"{{"translated_text":"{}"}}"#,
                    escape_json_string(&success.translated_text)
                )
            }
            TranslateResult::Failure(failure) => {
                format!(
                    r#"{{"code":"{}","message":"{}"}}"#,
                    failure.code.as_str(),
                    escape_json_string(&failure.message)
                )
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranslatableSegment {
    pub id: usize,
    pub text: String,
}

impl TranslatableSegment {
    pub fn new(id: usize, text: impl Into<String>) -> Result<Self, TranslateFailure> {
        let text = text.into();
        if text.len() > MAX_SEGMENT_BYTES {
            return Err(TranslateFailure::new(
                ErrorCode::FileTooLarge,
                "A translatable segment exceeds the configured size limit.",
            ));
        }
        Ok(Self { id, text })
    }
}

pub fn validate_segments(segments: &[TranslatableSegment]) -> Result<(), TranslateFailure> {
    if segments.is_empty() {
        return Err(TranslateFailure::new(
            ErrorCode::NoTranslatableSegments,
            "No translatable segments were found.",
        ));
    }
    if segments.len() > MAX_SEGMENTS {
        return Err(TranslateFailure::new(
            ErrorCode::FileTooLarge,
            "The request contains too many translatable segments.",
        ));
    }
    for segment in segments {
        if segment.text.len() > MAX_SEGMENT_BYTES {
            return Err(TranslateFailure::new(
                ErrorCode::FileTooLarge,
                "A translatable segment exceeds the configured size limit.",
            ));
        }
    }
    Ok(())
}

pub fn validate_direct_text_input(text: &str) -> Result<(), TranslateFailure> {
    if text.trim().is_empty() {
        return Err(TranslateFailure::new(
            ErrorCode::InvalidInput,
            "Input text must not be empty.",
        ));
    }
    validate_input_size(text)
}

fn validate_input_size(text: &str) -> Result<(), TranslateFailure> {
    if text.len() > MAX_INPUT_BYTES {
        return Err(TranslateFailure::new(
            ErrorCode::FileTooLarge,
            "The input exceeds the configured size limit.",
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum JsonValue {
    String(String),
    Bool(bool),
    Other,
}

struct JsonParser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> JsonParser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn parse_object(&mut self) -> Result<BTreeMap<String, JsonValue>, TranslateFailure> {
        self.skip_ws();
        self.expect_byte(b'{')?;
        self.skip_ws();

        let mut object = BTreeMap::new();
        if self.peek_byte() == Some(b'}') {
            self.pos += 1;
            self.finish()?;
            return Ok(object);
        }

        loop {
            self.skip_ws();
            let key = self.parse_string()?;
            self.skip_ws();
            self.expect_byte(b':')?;
            self.skip_ws();
            let value = self.parse_value()?;
            if object.insert(key, value).is_some() {
                return Err(TranslateFailure::invalid_input("Duplicate JSON field."));
            }
            self.skip_ws();
            match self.peek_byte() {
                Some(b',') => {
                    self.pos += 1;
                }
                Some(b'}') => {
                    self.pos += 1;
                    self.finish()?;
                    return Ok(object);
                }
                _ => return Err(TranslateFailure::invalid_input("Malformed JSON object.")),
            }
        }
    }

    fn parse_value(&mut self) -> Result<JsonValue, TranslateFailure> {
        match self.peek_byte() {
            Some(b'"') => Ok(JsonValue::String(self.parse_string()?)),
            Some(b't') => {
                self.expect_literal(b"true")?;
                Ok(JsonValue::Bool(true))
            }
            Some(b'f') => {
                self.expect_literal(b"false")?;
                Ok(JsonValue::Bool(false))
            }
            Some(b'n') => {
                self.expect_literal(b"null")?;
                Ok(JsonValue::Other)
            }
            Some(b'-' | b'0'..=b'9') => {
                self.consume_number();
                Ok(JsonValue::Other)
            }
            Some(b'{' | b'[') => Err(TranslateFailure::invalid_input(
                "Nested JSON values are not supported in TranslateRequest.",
            )),
            _ => Err(TranslateFailure::invalid_input("Malformed JSON value.")),
        }
    }

    fn parse_string(&mut self) -> Result<String, TranslateFailure> {
        self.expect_byte(b'"')?;
        let mut output = String::new();

        while let Some(byte) = self.peek_byte() {
            match byte {
                b'"' => {
                    self.pos += 1;
                    return Ok(output);
                }
                b'\\' => {
                    self.pos += 1;
                    output.push(self.parse_escape()?);
                }
                0x00..=0x1f => {
                    return Err(TranslateFailure::invalid_input(
                        "Control character in JSON string.",
                    ));
                }
                _ => output.push(self.next_char()?),
            }
        }

        Err(TranslateFailure::invalid_input("Unterminated JSON string."))
    }

    fn parse_escape(&mut self) -> Result<char, TranslateFailure> {
        match self.next_byte() {
            Some(b'"') => Ok('"'),
            Some(b'\\') => Ok('\\'),
            Some(b'/') => Ok('/'),
            Some(b'b') => Ok('\u{0008}'),
            Some(b'f') => Ok('\u{000c}'),
            Some(b'n') => Ok('\n'),
            Some(b'r') => Ok('\r'),
            Some(b't') => Ok('\t'),
            Some(b'u') => self.parse_unicode_escape(),
            _ => Err(TranslateFailure::invalid_input("Invalid JSON escape.")),
        }
    }

    fn parse_unicode_escape(&mut self) -> Result<char, TranslateFailure> {
        let value = self.parse_unicode_code_unit()?;
        match value {
            0xd800..=0xdbff => {
                self.expect_unicode_escape_prefix()?;
                let low = self.parse_unicode_code_unit()?;
                if !(0xdc00..=0xdfff).contains(&low) {
                    return Err(TranslateFailure::invalid_input("Invalid unicode."));
                }
                let scalar = 0x10000 + (((value - 0xd800) << 10) | (low - 0xdc00));
                char::from_u32(scalar)
                    .ok_or_else(|| TranslateFailure::invalid_input("Invalid unicode."))
            }
            0xdc00..=0xdfff => Err(TranslateFailure::invalid_input("Invalid unicode.")),
            _ => char::from_u32(value)
                .ok_or_else(|| TranslateFailure::invalid_input("Invalid unicode.")),
        }
    }

    fn parse_unicode_code_unit(&mut self) -> Result<u32, TranslateFailure> {
        let mut value = 0_u32;
        for _ in 0..4 {
            let byte = self
                .next_byte()
                .ok_or_else(|| TranslateFailure::invalid_input("Invalid unicode escape."))?;
            value = (value << 4)
                + match byte {
                    b'0'..=b'9' => (byte - b'0') as u32,
                    b'a'..=b'f' => (byte - b'a' + 10) as u32,
                    b'A'..=b'F' => (byte - b'A' + 10) as u32,
                    _ => {
                        return Err(TranslateFailure::invalid_input("Invalid unicode escape."));
                    }
                };
        }
        Ok(value)
    }

    fn expect_unicode_escape_prefix(&mut self) -> Result<(), TranslateFailure> {
        match (self.next_byte(), self.next_byte()) {
            (Some(b'\\'), Some(b'u')) => Ok(()),
            _ => Err(TranslateFailure::invalid_input("Invalid unicode escape.")),
        }
    }

    fn consume_number(&mut self) {
        while matches!(
            self.peek_byte(),
            Some(b'-' | b'+' | b'.' | b'0'..=b'9' | b'e' | b'E')
        ) {
            self.pos += 1;
        }
    }

    fn finish(&mut self) -> Result<(), TranslateFailure> {
        self.skip_ws();
        if self.pos == self.input.len() {
            Ok(())
        } else {
            Err(TranslateFailure::invalid_input(
                "Unexpected data after JSON object.",
            ))
        }
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek_byte(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.pos += 1;
        }
    }

    fn expect_byte(&mut self, expected: u8) -> Result<(), TranslateFailure> {
        match self.next_byte() {
            Some(byte) if byte == expected => Ok(()),
            _ => Err(TranslateFailure::invalid_input("Malformed JSON.")),
        }
    }

    fn expect_literal(&mut self, literal: &[u8]) -> Result<(), TranslateFailure> {
        if self
            .input
            .as_bytes()
            .get(self.pos..self.pos + literal.len())
            == Some(literal)
        {
            self.pos += literal.len();
            Ok(())
        } else {
            Err(TranslateFailure::invalid_input("Malformed JSON literal."))
        }
    }

    fn peek_byte(&self) -> Option<u8> {
        self.input.as_bytes().get(self.pos).copied()
    }

    fn next_byte(&mut self) -> Option<u8> {
        let byte = self.peek_byte()?;
        self.pos += 1;
        Some(byte)
    }

    fn next_char(&mut self) -> Result<char, TranslateFailure> {
        let ch = self.input[self.pos..]
            .chars()
            .next()
            .ok_or_else(|| TranslateFailure::invalid_input("Malformed JSON string."))?;
        self.pos += ch.len_utf8();
        Ok(ch)
    }
}

fn parse_json_object(input: &str) -> Result<BTreeMap<String, JsonValue>, TranslateFailure> {
    JsonParser::new(input).parse_object()
}

fn reject_unknown_fields(object: &BTreeMap<String, JsonValue>) -> Result<(), TranslateFailure> {
    const ALLOWED: [&str; 9] = [
        "source_text",
        "source_language",
        "target_language",
        "tone",
        "preserve_formatting",
        "input_kind",
        "file_path",
        "workspace_root",
        "remote_confirmed",
    ];

    for key in object.keys() {
        if !ALLOWED.contains(&key.as_str()) {
            return Err(TranslateFailure::invalid_input("Unknown request field."));
        }
    }
    Ok(())
}

fn required_string<'a>(
    object: &'a BTreeMap<String, JsonValue>,
    key: &str,
) -> Result<&'a str, TranslateFailure> {
    match object.get(key) {
        Some(JsonValue::String(value)) => Ok(value),
        Some(_) => Err(TranslateFailure::invalid_input(
            "JSON field has the wrong type.",
        )),
        None => Err(TranslateFailure::invalid_input(
            "Missing required JSON field.",
        )),
    }
}

fn optional_string(
    object: &BTreeMap<String, JsonValue>,
    key: &str,
) -> Result<Option<String>, TranslateFailure> {
    match object.get(key) {
        Some(JsonValue::String(value)) => Ok(Some(value.clone())),
        Some(_) => Err(TranslateFailure::invalid_input(
            "JSON field has the wrong type.",
        )),
        None => Ok(None),
    }
}

fn required_bool(
    object: &BTreeMap<String, JsonValue>,
    key: &str,
) -> Result<bool, TranslateFailure> {
    match object.get(key) {
        Some(JsonValue::Bool(value)) => Ok(*value),
        Some(_) => Err(TranslateFailure::invalid_input(
            "JSON field has the wrong type.",
        )),
        None => Err(TranslateFailure::invalid_input(
            "Missing required JSON field.",
        )),
    }
}

fn optional_bool(
    object: &BTreeMap<String, JsonValue>,
    key: &str,
) -> Result<Option<bool>, TranslateFailure> {
    match object.get(key) {
        Some(JsonValue::Bool(value)) => Ok(Some(*value)),
        Some(_) => Err(TranslateFailure::invalid_input(
            "JSON field has the wrong type.",
        )),
        None => Ok(None),
    }
}

fn push_string_field(json: &mut String, first: &mut bool, key: &str, value: &str) {
    push_separator(json, first);
    json.push('"');
    json.push_str(key);
    json.push_str("\":\"");
    json.push_str(&escape_json_string(value));
    json.push('"');
}

fn push_bool_field(json: &mut String, first: &mut bool, key: &str, value: bool) {
    push_separator(json, first);
    json.push('"');
    json.push_str(key);
    json.push_str("\":");
    json.push_str(if value { "true" } else { "false" });
}

fn push_separator(json: &mut String, first: &mut bool) {
    if *first {
        *first = false;
    } else {
        json.push(',');
    }
}

fn escape_json_string(value: &str) -> String {
    let mut output = String::new();
    for ch in value.chars() {
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{0008}' => output.push_str("\\b"),
            '\u{000c}' => output.push_str("\\f"),
            ch if ch <= '\u{001f}' => {
                output.push_str(&format!("\\u{:04x}", ch as u32));
            }
            ch => output.push(ch),
        }
    }
    output
}
