//! Re-export the prelude types

pub use codec;
pub use service::service::WorkPackage;

#[cfg(feature = "std")]
pub use std::{string::String, vec, vec::Vec};

#[cfg(not(feature = "std"))]
pub use alloc::{string::String, vec, vec::Vec};

/// Type to represent the index of a compute core.
pub type CoreIndex = u32;

/// Type to represent the authorizer configuration.
pub type AuthConfig = Vec<u8>;

/// Type to represent the authorizer trace.
pub type AuthTrace = Vec<u8>;
