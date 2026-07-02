use std::fs::{self, File, Metadata};
use std::io::Read;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::path::{Component, Path};

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
    load_allowed_file_with_open_hook(file_path, workspace_root, || {})
}

fn load_allowed_file_with_open_hook(
    file_path: &str,
    workspace_root: &str,
    before_open: impl FnOnce(),
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
    let validated_metadata = fs::metadata(&canonical_file).map_err(|_| {
        TranslateFailure::new(ErrorCode::FileNotFound, "The requested file was not found.")
    })?;

    before_open();

    let file = File::open(&canonical_file).map_err(|_| {
        TranslateFailure::new(ErrorCode::FileNotFound, "The requested file was not found.")
    })?;
    let metadata = file.metadata().map_err(|_| {
        TranslateFailure::new(ErrorCode::FileNotFound, "The requested file was not found.")
    })?;
    if !metadata.is_file() {
        return Err(path_not_allowed());
    }
    if !same_file(&validated_metadata, &metadata) {
        return Err(path_not_allowed());
    }
    if metadata.len() > MAX_INPUT_BYTES as u64 {
        return Err(TranslateFailure::new(
            ErrorCode::FileTooLarge,
            "The input exceeds the configured size limit.",
        ));
    }

    let mut bytes = Vec::new();
    let mut limited_file = file.take((MAX_INPUT_BYTES + 1) as u64);
    limited_file.read_to_end(&mut bytes).map_err(|_| {
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

#[cfg(unix)]
fn same_file(validated: &Metadata, opened: &Metadata) -> bool {
    validated.dev() == opened.dev() && validated.ino() == opened.ino()
}

#[cfg(not(unix))]
fn same_file(_: &Metadata, _: &Metadata) -> bool {
    true
}

fn path_not_allowed() -> TranslateFailure {
    TranslateFailure::new(
        ErrorCode::PathNotAllowed,
        "The requested path is not allowed.",
    )
}

#[cfg(all(test, unix))]
mod tests {
    use std::fs;
    use std::os::unix::fs::symlink;
    use std::path::{Path, PathBuf};

    use super::load_allowed_file_with_open_hook;
    use crate::ErrorCode;

    #[test]
    fn rejects_replaced_validated_target_before_opening_file() {
        let root = temp_case("toctou_after_validation");
        let workspace = root.join("ws");
        let inside = workspace.join("inside.md");
        let outside = root.join("outside.md");
        let link = workspace.join("doc.md");
        fs::create_dir_all(&workspace).expect("workspace");
        write_file(&inside, "Read the docs.");
        write_file(&outside, "Open the file.");
        symlink(&inside, &link).expect("initial inside symlink");

        let err = load_allowed_file_with_open_hook("doc.md", workspace.to_str().unwrap(), || {
            fs::remove_file(&inside).expect("remove validated target");
            symlink(&outside, &inside).expect("replace target with outside symlink");
        })
        .expect_err("replaced validated target should fail");

        assert_eq!(err.code, ErrorCode::PathNotAllowed);
    }

    fn temp_case(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "zed_translator_{name}_{}_{}",
            std::process::id(),
            unique_suffix()
        ));
        fs::create_dir_all(&root).expect("temp root");
        root
    }

    fn unique_suffix() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos()
    }

    fn write_file(path: &Path, content: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("parent dir");
        }
        fs::write(path, content).expect("write file");
    }
}
