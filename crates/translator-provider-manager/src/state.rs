use serde::{Deserialize, Serialize};

use crate::error::ManagerError;

/// Atomic logical references for immutable embedded artifact sets.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InstallationState {
    schema_version: u32,
    generation: u64,
    profile_id: String,
    current: Option<String>,
    previous: Option<String>,
    candidate: Option<String>,
    last_operation: String,
    last_outcome: String,
}

impl InstallationState {
    /// Create an empty state for one fixed product profile.
    pub fn empty(profile_id: impl Into<String>) -> Self {
        Self {
            schema_version: 1,
            generation: 0,
            profile_id: profile_id.into(),
            current: None,
            previous: None,
            candidate: None,
            last_operation: "none".to_string(),
            last_outcome: "ready".to_string(),
        }
    }

    /// Parse strict state JSON and validate all logical references.
    ///
    /// # Errors
    ///
    /// Returns [`ManagerError::StateInvalid`] for an unknown schema, unsafe
    /// profile, invalid digest, or contradictory references.
    pub fn from_json(input: &str) -> Result<Self, ManagerError> {
        let state: Self = serde_json::from_str(input).map_err(|_| ManagerError::StateInvalid)?;
        state.validate()?;
        Ok(state)
    }

    /// Serialize state for an atomic replacement file.
    ///
    /// # Errors
    ///
    /// Returns a stable state error if serialization unexpectedly fails.
    pub fn to_json(&self) -> Result<Vec<u8>, ManagerError> {
        serde_json::to_vec(self).map_err(|_| ManagerError::StateInvalid)
    }

    /// Stage one fully validated immutable candidate digest.
    pub fn stage_candidate(&mut self, digest: &str) -> Result<(), ManagerError> {
        if !is_sha256(digest) || self.candidate.is_some() || self.current.as_deref() == Some(digest)
        {
            return Err(ManagerError::StateInvalid);
        }
        self.candidate = Some(digest.to_string());
        self.record("stage", "candidate_ready")
    }

    /// Promote the staged candidate and retain the old current as previous.
    pub fn promote_candidate(&mut self) -> Result<(), ManagerError> {
        let candidate = self.candidate.take().ok_or(ManagerError::StateInvalid)?;
        self.previous = self.current.replace(candidate);
        self.record("promote", "ready")
    }

    /// Reject a staged candidate without changing current/previous.
    pub fn reject_candidate(&mut self) -> Result<(), ManagerError> {
        if self.candidate.take().is_none() {
            return Err(ManagerError::StateInvalid);
        }
        self.record("reject", "candidate_rejected")
    }

    /// Revalidate externally, then atomically swap current and previous.
    pub fn rollback(&mut self) -> Result<(), ManagerError> {
        let previous = self.previous.take().ok_or(ManagerError::StateInvalid)?;
        self.previous = self.current.replace(previous);
        self.record("rollback", "ready")
    }

    /// Return current, previous, and candidate digest references.
    pub fn references(&self) -> (Option<&str>, Option<&str>, Option<&str>) {
        (
            self.current.as_deref(),
            self.previous.as_deref(),
            self.candidate.as_deref(),
        )
    }

    /// Return the monotonically increasing state generation.
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Return the fixed profile identifier.
    pub fn profile_id(&self) -> &str {
        &self.profile_id
    }

    fn validate(&self) -> Result<(), ManagerError> {
        if self.schema_version != 1
            || self.profile_id != "bergamot-en-es-linux-x86_64-v1"
            || !self.current.as_deref().is_none_or(is_sha256)
            || !self.previous.as_deref().is_none_or(is_sha256)
            || !self.candidate.as_deref().is_none_or(is_sha256)
            || self.current == self.candidate
            || self.current == self.previous
        {
            return Err(ManagerError::StateInvalid);
        }
        Ok(())
    }

    fn record(&mut self, operation: &str, outcome: &str) -> Result<(), ManagerError> {
        self.generation = self
            .generation
            .checked_add(1)
            .ok_or(ManagerError::StateInvalid)?;
        self.last_operation = operation.to_string();
        self.last_outcome = outcome.to_string();
        Ok(())
    }
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
