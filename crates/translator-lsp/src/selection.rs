//! Checked conversion and target resolution for editor selections.

use std::ops::Range as ByteRange;
use std::path::PathBuf;

use lsp_types::Uri;
use lsp_types::{Position, Range};
use serde::{Deserialize, Serialize};
use translator_core::InputKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionError {
    InvalidPosition,
    ReversedRange,
    InvalidUri,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TranslationTarget {
    pub uri: Uri,
    pub version: i32,
    pub range: Range,
    pub input_kind: String,
}

impl TranslationTarget {
    pub fn new(uri: Uri, version: i32, range: Range, input_kind: InputKind) -> Self {
        Self {
            uri,
            version,
            range,
            input_kind: input_kind.as_str().to_string(),
        }
    }

    pub fn parsed_input_kind(&self) -> Result<InputKind, SelectionError> {
        match self.input_kind.as_str() {
            "text" => Ok(InputKind::Text),
            "markdown" => Ok(InputKind::Markdown),
            _ => Err(SelectionError::InvalidPosition),
        }
    }
}

pub fn position_to_offset(text: &str, position: Position) -> Result<usize, SelectionError> {
    let line_start = line_start(text, position.line)?;
    let mut line_end = text[line_start..]
        .find('\n')
        .map_or(text.len(), |relative| line_start + relative);
    if line_end > line_start && text.as_bytes()[line_end - 1] == b'\r' {
        line_end -= 1;
    }

    let target = position.character;
    let mut utf16_units = 0_u32;
    let mut offset = line_start;
    for ch in text[line_start..line_end].chars() {
        if utf16_units == target {
            return Ok(offset);
        }
        let next = utf16_units + ch.len_utf16() as u32;
        if target < next {
            return Err(SelectionError::InvalidPosition);
        }
        utf16_units = next;
        offset += ch.len_utf8();
    }

    (utf16_units == target)
        .then_some(offset)
        .ok_or(SelectionError::InvalidPosition)
}

pub fn range_to_offsets(text: &str, range: Range) -> Result<ByteRange<usize>, SelectionError> {
    let start = position_to_offset(text, range.start)?;
    let end = position_to_offset(text, range.end)?;
    if start > end {
        return Err(SelectionError::ReversedRange);
    }
    Ok(start..end)
}

pub fn file_path_from_uri(uri: &Uri) -> Result<PathBuf, SelectionError> {
    let parsed = url::Url::parse(uri.as_str()).map_err(|_| SelectionError::InvalidUri)?;
    if parsed.scheme() != "file" || parsed.query().is_some() || parsed.fragment().is_some() {
        return Err(SelectionError::InvalidUri);
    }
    parsed
        .to_file_path()
        .map_err(|()| SelectionError::InvalidUri)
}

pub fn full_document_range(text: &str) -> Range {
    let line = text.bytes().filter(|byte| *byte == b'\n').count() as u32;
    let last_line = text.rsplit_once('\n').map_or(text, |(_, line)| line);
    let last_line = last_line.strip_suffix('\r').unwrap_or(last_line);
    let character = last_line.chars().map(|ch| ch.len_utf16() as u32).sum();
    Range::new(Position::new(0, 0), Position::new(line, character))
}

fn line_start(text: &str, target_line: u32) -> Result<usize, SelectionError> {
    if target_line == 0 {
        return Ok(0);
    }

    let mut current_line = 0_u32;
    for (offset, byte) in text.bytes().enumerate() {
        if byte == b'\n' {
            current_line += 1;
            if current_line == target_line {
                return Ok(offset + 1);
            }
        }
    }
    Err(SelectionError::InvalidPosition)
}
