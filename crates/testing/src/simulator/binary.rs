//! PVM binary simulator

use crate::{env::Env, simulator::Execution};
use anyhow::Result;
use podec::{Decode, Encode};
use std::process::{Command, Stdio};

/// The PVM binary simulator
#[derive(Debug, Clone)]
pub struct Simulator {
    command: String,
}

impl Simulator {
    /// Create a new PVM binary simulator
    pub fn new(command: String) -> Self {
        Self { command }
    }

    /// Run the is-authorized interface
    pub fn is_authorized(&self, env: &Env) -> Result<Execution> {
        let encoded = hex::encode(env.encode());
        let output = Command::new(&self.command)
            .arg(encoded)
            .arg("authorize")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        if !output.stderr.is_empty() {
            anyhow::bail!(
                "Failed to run is-authorized: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        let encoded = hex::decode(output.stdout)?;
        Execution::decode(&mut encoded.as_slice())
            .map_err(|e| anyhow::anyhow!("Failed to decode is-authorized result: {e}"))
    }

    /// Run the refine interface
    pub fn refine(&self, env: &Env) -> Result<Execution> {
        let encoded = hex::encode(env.encode());
        let output = Command::new(&self.command)
            .arg(encoded)
            .arg("refine")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        if !output.stderr.is_empty() {
            anyhow::bail!(
                "Failed to run refine: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        let encoded = hex::decode(output.stdout)?;
        Execution::decode(&mut encoded.as_slice())
            .map_err(|e| anyhow::anyhow!("Failed to decode refine result: {e}"))
    }

    /// Run the accumulate interface
    pub fn accumulate(&self, env: &Env) -> Result<Execution> {
        let encoded = hex::encode(env.encode());
        let output = Command::new(&self.command)
            .arg(encoded)
            .arg("accumulate")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        if !output.stderr.is_empty() {
            anyhow::bail!("{}", String::from_utf8_lossy(&output.stderr));
        }

        let encoded = hex::decode(output.stdout)?;
        Execution::decode(&mut encoded.as_slice())
            .map_err(|e| anyhow::anyhow!("Failed to decode accumulate result: {e}"))
    }
}

impl Default for Simulator {
    fn default() -> Self {
        Self::new("pvms".to_string())
    }
}
