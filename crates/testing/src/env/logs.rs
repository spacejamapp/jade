//! Logs of the execution

use podec::{Decode, Encode};

/// logs of the execution
#[derive(Encode, Decode, Default, Clone)]
pub struct Logs {
    /// The logs of the is_authorized step
    pub is_authorized: Vec<String>,

    /// The logs of the refine step
    pub refine: Vec<String>,

    /// The logs of the accumulate step
    pub accumulate: Vec<String>,
}
