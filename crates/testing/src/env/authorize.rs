//! Authorize environment

use podec::{Decode, Encode};

/// Authorize environment
#[derive(Encode, Decode)]
pub struct Authorize {
    /// The code of the service
    pub code: Vec<u8>,

    /// The index of the core
    pub index: u16,
}
