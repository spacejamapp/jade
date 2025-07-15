//! PVM execution result

use std::borrow::Cow;

use crate::env::Env;
use anyhow::Result;
use hex;
use podec::{Decode, Encode};

/// Execution result
#[derive(Encode, Decode)]
pub struct Execution {
    /// The logs of the execution
    pub logs: Vec<String>,

    /// The environment after the execution
    pub env: Env,
}

impl Execution {
    /// Parse from stdout
    pub fn from_stdout(stdout: Cow<'_, str>) -> Result<Self> {
        let parts: Vec<&str> = stdout.split("--encoded--").collect();
        if parts.len() != 2 {
            anyhow::bail!(
                "Failed to split stdout: expected 2 parts, got {}",
                parts.len()
            );
        }

        let logs = parts[0]
            .split('\n')
            .filter_map(|s| {
                if s.is_empty() {
                    None
                } else {
                    Some(s.to_string())
                }
            })
            .collect::<Vec<_>>();

        let encoded = hex::decode(parts[1])?;
        Ok(Execution {
            logs,
            env: Env::decode(&mut encoded.as_slice())
                .map_err(|e| anyhow::anyhow!("Failed to decode is-authorized result: {e}"))?,
        })
    }
}
