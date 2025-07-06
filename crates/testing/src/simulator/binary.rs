//! PVM binary simulator

use crate::env::Env;
use anyhow::Result;
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
    pub fn is_authorized(&self, env: &Env) -> Result<()> {
        Command::new(&self.command)
            .arg("is-authorized")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        Ok(())
    }

    /// Run the refine interface
    pub fn refine(&self, env: &Env) -> Result<()> {
        Command::new(&self.command)
            .arg("refine")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
            .wait()?;

        Ok(())
    }

    /// Run the accumulate interface
    pub fn accumulate(&self, env: &Env) -> Result<()> {
        Command::new(&self.command)
            .arg("accumulate")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        Ok(())
    }
}

impl Default for Simulator {
    fn default() -> Self {
        Self::new("pvms".to_string())
    }
}
