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
            .arg("is-authorized")
            .arg(encoded)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        Execution::decode(&mut output.stdout.as_slice()).map_err(Into::into)
    }

    /// Run the refine interface
    pub fn refine(&self, env: &Env) -> Result<Execution> {
        let encoded = hex::encode(env.encode());
        let output = Command::new(&self.command)
            .arg("refine")
            .arg(encoded)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        Execution::decode(&mut output.stdout.as_slice()).map_err(Into::into)
    }

    /// Run the accumulate interface
    pub fn accumulate(&self, env: &Env) -> Result<Execution> {
        let encoded = hex::encode(env.encode());
        let output = Command::new(&self.command)
            .arg("accumulate")
            .arg(encoded)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        Execution::decode(&mut output.stdout.as_slice()).map_err(Into::into)
    }
}

impl Default for Simulator {
    fn default() -> Self {
        Self::new("pvms".to_string())
    }
}
