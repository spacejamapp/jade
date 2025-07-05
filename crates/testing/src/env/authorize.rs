//! Authorize environment

/// Authorize environment
pub struct Authorize {
    /// The code of the service
    pub code: Vec<u8>,

    /// The index of the core
    pub index: u16,
}
