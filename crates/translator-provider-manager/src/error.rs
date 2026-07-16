use thiserror::Error;

/// Content-free lifecycle failures suitable for normalized diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ManagerError {
    #[error("The embedded provider manifest is invalid.")]
    ManifestInvalid,
    #[error("The embedded provider requires a matching human approval.")]
    ApprovalRequired,
    #[error("The embedded provider state is invalid.")]
    StateInvalid,
    #[error("The embedded provider storage location is unsafe.")]
    StorageUnsafe,
    #[error("The embedded provider storage operation failed.")]
    StorageFailed,
    #[error("Exact consent is required for this embedded provider manifest.")]
    ConsentRequired,
    #[error("The embedded provider acquisition failed.")]
    AcquisitionFailed,
    #[error("The embedded provider artifact identity is invalid.")]
    IntegrityFailed,
    #[error("The embedded provider lifecycle is busy.")]
    Busy,
    #[error("The embedded provider cleanup scope is ambiguous.")]
    CleanupRefused,
}

impl ManagerError {
    /// Return the stable machine-readable failure class.
    pub const fn code(self) -> &'static str {
        match self {
            Self::ManifestInvalid => "MANIFEST_INVALID",
            Self::ApprovalRequired => "APPROVAL_REQUIRED",
            Self::StateInvalid => "STATE_INVALID",
            Self::StorageUnsafe => "STORAGE_UNSAFE",
            Self::StorageFailed => "STORAGE_FAILED",
            Self::ConsentRequired => "CONSENT_REQUIRED",
            Self::AcquisitionFailed => "ACQUISITION_FAILED",
            Self::IntegrityFailed => "INTEGRITY_FAILED",
            Self::Busy => "BUSY",
            Self::CleanupRefused => "CLEANUP_REFUSED",
        }
    }
}
