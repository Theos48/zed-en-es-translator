use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::{ErrorCode, InputKind, TranslateFailure, MAX_INPUT_BYTES};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedFile {
    pub content: String,
    pub input_kind: InputKind,
}

pub fn load_allowed_file(
    file_path: &str,
    workspace_root: &str,
) -> Result<LoadedFile, TranslateFailure> {
    let requested = Path::new(file_path);
    if has_parent_component(requested) {
        return Err(path_not_allowed());
    }
    if is_sensitive_path(requested) {
        return Err(path_not_allowed());
    }

    let input_kind = input_kind_from_path(requested)?;
    let root = fs::canonicalize(workspace_root).map_err(|_| path_not_allowed())?;
    let candidate = if requested.is_absolute() {
        requested.to_path_buf()
    } else {
        root.join(requested)
    };
    let canonical_file = fs::canonicalize(&candidate).map_err(|_| {
        TranslateFailure::new(ErrorCode::FileNotFound, "The requested file was not found.")
    })?;

    if !canonical_file.starts_with(&root) {
        return Err(path_not_allowed());
    }
    if is_sensitive_path(
        canonical_file
            .strip_prefix(&root)
            .unwrap_or(&canonical_file),
    ) {
        return Err(path_not_allowed());
    }

    let metadata = fs::metadata(&canonical_file).map_err(|_| {
        TranslateFailure::new(ErrorCode::FileNotFound, "The requested file was not found.")
    })?;
    if !metadata.is_file() {
        return Err(path_not_allowed());
    }
    if metadata.len() > MAX_INPUT_BYTES as u64 {
        return Err(TranslateFailure::new(
            ErrorCode::FileTooLarge,
            "The input exceeds the configured size limit.",
        ));
    }

    let bytes = fs::read(&canonical_file).map_err(|_| {
        TranslateFailure::new(ErrorCode::FileNotFound, "The requested file was not found.")
    })?;
    if bytes.len() > MAX_INPUT_BYTES {
        return Err(TranslateFailure::new(
            ErrorCode::FileTooLarge,
            "The input exceeds the configured size limit.",
        ));
    }
    if looks_binary(&bytes) {
        return Err(TranslateFailure::new(
            ErrorCode::NonUtf8Input,
            "The input must be UTF-8 text.",
        ));
    }

    let content = String::from_utf8(bytes).map_err(|_| {
        TranslateFailure::new(ErrorCode::NonUtf8Input, "The input must be UTF-8 text.")
    })?;

    Ok(LoadedFile {
        content,
        input_kind,
    })
}

fn input_kind_from_path(path: &Path) -> Result<InputKind, TranslateFailure> {
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    match extension.as_str() {
        "md" | "markdown" => Ok(InputKind::Markdown),
        "txt" => Ok(InputKind::Text),
        _ => Err(TranslateFailure::new(
            ErrorCode::UnsupportedFileType,
            "The requested file type is not supported.",
        )),
    }
}

fn has_parent_component(path: &Path) -> bool {
    path.components()
        .any(|component| matches!(component, Component::ParentDir))
}

fn is_sensitive_path(path: &Path) -> bool {
    path.components().any(|component| match component {
        Component::Normal(name) => name.to_str().map(is_sensitive_name).unwrap_or(true),
        _ => false,
    })
}

fn is_sensitive_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower == ".env"
        || lower.starts_with(".env.")
        || lower.contains("credential")
        || lower.contains("secret")
        || lower.contains("token")
        || lower.contains("private_key")
        || lower == "id_rsa"
        || lower == "id_ed25519"
}

fn looks_binary(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .any(|byte| *byte == 0 || (*byte < 0x20 && !matches!(*byte, b'\n' | b'\r' | b'\t')))
}

fn path_not_allowed() -> TranslateFailure {
    TranslateFailure::new(
        ErrorCode::PathNotAllowed,
        "The requested path is not allowed.",
    )
}

#[allow(dead_code)]
fn _pathbuf_for_docs(_: PathBuf) {}
