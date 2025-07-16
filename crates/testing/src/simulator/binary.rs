//! PVM binary simulator

use crate::{env::Env, simulator::Execution};
use anyhow::Result;
use podec::Encode;
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
            .arg(env.target.clone())
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

        let stdout = String::from_utf8_lossy(&output.stdout);
        Execution::from_stdout(stdout)
    }

    /// Run the refine interface
    pub fn refine(&self, env: &Env) -> Result<Execution> {
        let encoded = hex::encode(env.encode());
        let output = Command::new(&self.command)
            .arg(env.target.clone())
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

        Execution::from_stdout(String::from_utf8_lossy(&output.stdout))
    }

    /// Run the accumulate interface
    pub fn accumulate(&self, env: &Env) -> Result<Execution> {
        let encoded = hex::encode(env.encode());
        let output = Command::new(&self.command)
            .arg(env.target.clone())
            .arg(encoded)
            .arg("accumulate")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        if !output.stderr.is_empty() {
            anyhow::bail!("{}", String::from_utf8_lossy(&output.stderr));
        }

        Execution::from_stdout(String::from_utf8_lossy(&output.stdout))
    }
}

impl Default for Simulator {
    fn default() -> Self {
        Self::new("pvms".to_string())
    }
}
