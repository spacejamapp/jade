//! PVM execution result

use crate::env::Env;
use podec::{Decode, Encode};

/// Execution result
#[derive(Encode, Decode)]
pub struct Execution {
    /// The logs of the execution
    pub logs: Vec<String>,

    /// The environment after the execution
    pub env: Env,
}
