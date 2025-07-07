//! PVM execution result

use crate::env::Env;

/// Execution result
pub struct Execution {
    /// The logs of the execution
    pub logs: Vec<String>,

    /// The environment after the execution
    pub env: Env,

    /// The output of the execution
    pub output: Option<Vec<u8>>,
}
