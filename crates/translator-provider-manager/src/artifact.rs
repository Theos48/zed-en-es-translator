use std::io::Read;

use sha2::{Digest as _, Sha256};

use crate::error::ManagerError;

/// Expand one Zstandard attachment with exact installed size and identity.
///
/// # Errors
///
/// Corrupt input, excess/truncated output, and hash mismatches fail closed.
pub fn expand_zstandard(
    source: impl Read,
    expected_size: usize,
    expected_sha256: &str,
) -> Result<Vec<u8>, ManagerError> {
    if expected_size == 0 || expected_size > 128 * 1024 * 1024 || !is_sha256(expected_sha256) {
        return Err(ManagerError::ManifestInvalid);
    }
    let mut decoder = zstd::Decoder::new(source).map_err(|_| ManagerError::IntegrityFailed)?;
    let mut output = Vec::with_capacity(expected_size);
    decoder
        .by_ref()
        .take((expected_size + 1) as u64)
        .read_to_end(&mut output)
        .map_err(|_| ManagerError::IntegrityFailed)?;
    if output.len() != expected_size || sha256(&output) != expected_sha256 {
        return Err(ManagerError::IntegrityFailed);
    }
    Ok(output)
}

fn sha256(content: &[u8]) -> String {
    Sha256::digest(content)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
