//! PVM execution result

use crate::env::Env;
use jam_types::WorkError;
use podec::{Decode, Encode};

/// Execution result
#[derive(Encode, Decode)]
pub struct Execution {
    /// The logs of the execution
    pub logs: Vec<String>,

    /// The environment after the execution
    pub env: Env,

    /// The output of the execution
    pub output: Result<Vec<u8>, WorkError>,
}
