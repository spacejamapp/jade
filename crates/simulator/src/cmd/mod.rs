//! Command line interface

use anyhow::Result;
use clap::Parser;
use podec::{Decode, Encode};
use std::io::{self, Write};
use testing::Env;
use tracing_subscriber::EnvFilter;

mod accumulate;
mod authorize;
mod refine;

/// The interface of the PVM simulator
///
/// NOTE: this should only be used internally.
#[derive(Debug, Parser)]
pub struct App {
    /// The command to run
    #[clap(subcommand)]
    command: Command,

    /// The target to filter the logs
    target: String,

    /// The environment to run the command on
    env: String,
}

impl App {
    /// Run the application
    pub fn run() -> Result<()> {
        let app = Self::parse();
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or(EnvFilter::new(format!("warn,{}=trace", app.target)));

        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_ansi(false)
            .without_time()
            .init();

        let decoded = hex::decode(app.env)?;
        let env = Env::decode(&mut decoded.as_slice())
            .map_err(|e| anyhow::anyhow!("failed to decode env: {e}"))?;

        let exec = match app.command {
            Command::Refine => refine::run(&env),
            Command::Accumulate => accumulate::run(&env),
            Command::Authorize => authorize::run(&env),
        }?;

        io::stdout().write_all(b"--encoded--")?;
        io::stdout().write_all(hex::encode(exec.encode()).as_bytes())?;
        Ok(())
    }
}

/// The command to run
#[derive(Debug, Parser, Clone)]
pub enum Command {
    /// Refine the environment
    Refine,

    /// Accumulate the environment
    Accumulate,

    /// Authorize the environment
    Authorize,
}
