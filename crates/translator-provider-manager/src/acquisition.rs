use std::io::Read;
use std::time::Duration;

use sha2::{Digest as _, Sha256};

use crate::error::ManagerError;

const MAX_ARTIFACT_BYTES: usize = 64 * 1024 * 1024;
const STRICT_TRANSPORT: AcquisitionTransportPolicy = AcquisitionTransportPolicy {
    https_only: true,
    expected_status: 200,
    max_redirects: 0,
    inherited_proxy: false,
    retries: 0,
};

/// Reviewable transport invariants used by every production acquisition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AcquisitionTransportPolicy {
    https_only: bool,
    expected_status: u16,
    max_redirects: u32,
    inherited_proxy: bool,
    retries: u32,
}

impl AcquisitionTransportPolicy {
    pub const fn https_only(self) -> bool {
        self.https_only
    }

    pub const fn expected_status(self) -> u16 {
        self.expected_status
    }

    pub const fn max_redirects(self) -> u32 {
        self.max_redirects
    }

    pub const fn inherited_proxy(self) -> bool {
        self.inherited_proxy
    }

    pub const fn retries(self) -> u32 {
        self.retries
    }
}

/// Exact source, length, and identity required for one acquisition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcquisitionPolicy {
    url: String,
    expected_size: usize,
    expected_sha256: String,
}

impl AcquisitionPolicy {
    /// Return the fixed transport invariants applied by `acquire_https`.
    pub const fn transport_policy(&self) -> AcquisitionTransportPolicy {
        STRICT_TRANSPORT
    }

    /// Validate an exact HTTPS acquisition policy.
    ///
    /// # Errors
    ///
    /// Rejects unsafe URLs, zero/oversized lengths, and invalid digests.
    pub fn new(
        url: &str,
        expected_size: usize,
        expected_sha256: &str,
    ) -> Result<Self, ManagerError> {
        if !url.starts_with("https://")
            || url.contains('@')
            || url.contains('#')
            || expected_size == 0
            || expected_size > MAX_ARTIFACT_BYTES
            || !is_sha256(expected_sha256)
        {
            return Err(ManagerError::ManifestInvalid);
        }
        Ok(Self {
            url: url.to_string(),
            expected_size,
            expected_sha256: expected_sha256.to_string(),
        })
    }

    /// Read one bounded body and validate its declared and observed identity.
    ///
    /// # Errors
    ///
    /// Missing/mismatched length, read failure, truncation, excess data, and
    /// hash mismatch return a content-free acquisition/integrity error.
    pub fn verify_reader(
        &self,
        mut reader: impl Read,
        content_length: Option<usize>,
    ) -> Result<Vec<u8>, ManagerError> {
        if content_length != Some(self.expected_size) {
            return Err(ManagerError::AcquisitionFailed);
        }
        let mut bytes = Vec::with_capacity(self.expected_size);
        reader
            .by_ref()
            .take((self.expected_size + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(|_| ManagerError::AcquisitionFailed)?;
        if bytes.len() != self.expected_size {
            return Err(ManagerError::AcquisitionFailed);
        }
        if sha256(&bytes) != self.expected_sha256 {
            return Err(ManagerError::IntegrityFailed);
        }
        Ok(bytes)
    }

    /// Acquire the exact URL with proxy inheritance and redirects disabled.
    ///
    /// # Errors
    ///
    /// Any transport, status, length, size, or identity mismatch is normalized.
    pub fn acquire_https(&self) -> Result<Vec<u8>, ManagerError> {
        let transport = self.transport_policy();
        let config = ureq::Agent::config_builder()
            .https_only(transport.https_only())
            .proxy(None)
            .max_redirects(transport.max_redirects())
            .timeout_global(Some(Duration::from_secs(60)))
            .build();
        let agent = ureq::Agent::new_with_config(config);
        let mut response = agent
            .get(&self.url)
            .call()
            .map_err(|_| ManagerError::AcquisitionFailed)?;
        if response.status().as_u16() != transport.expected_status() {
            return Err(ManagerError::AcquisitionFailed);
        }
        let content_length = response
            .headers()
            .get("content-length")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<usize>().ok());
        self.verify_reader(response.body_mut().as_reader(), content_length)
    }
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
