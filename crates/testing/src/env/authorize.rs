//! Authorize environment

use podec::{Decode, Encode};

const NULL_AUTHORIZER: &[u8] =
    include_bytes!("../../../../services/jam-null-authorizer/jam-null-authorizer.jam");

/// Authorize environment
#[derive(Encode, Decode, Clone)]
pub struct Authorize {
    /// The code of the service
    pub code: Vec<u8>,

    /// The index of the core
    pub index: u16,
}

impl Authorize {
    /// Create a null authorizer
    pub fn null() -> Self {
        Self {
            code: NULL_AUTHORIZER.to_vec(),
            index: 256,
        }
    }
}

impl Default for Authorize {
    fn default() -> Self {
        Self::null()
    }
}
