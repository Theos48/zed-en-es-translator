//! Lifecycle management for the reviewed embedded translation provider.

pub mod acquisition;
pub mod artifact;
pub mod cleanup;
pub mod cli;
pub mod disclosure;
pub mod error;
pub mod lifecycle;
pub mod locking;
pub mod manifest;
pub mod state;
pub mod status;
pub mod storage;

/// Returns the lifecycle protocol version implemented by this crate.
pub const fn protocol_version() -> u32 {
    1
}
