//! Command line interface

use anyhow::Result;
use clap::Parser;
use podec::{Decode, Encode};
use std::io::{self, Write};
use testing::Env;

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

    /// The environment to run the command on
    #[clap(short, long)]
    env: Vec<u8>,
}

impl App {
    /// Run the application
    pub fn run() -> Result<()> {
        let app = Self::parse();
        let decoded = hex::decode(app.env)?;
        let mut env = Env::decode(&mut decoded.as_slice())?;
        let exec = match app.command {
            Command::Refine => refine::run(&mut env),
            Command::Accumulate => accumulate::run(&env),
            Command::Authorize => authorize::run(&env),
        }?;

        io::stdout().write_all(&exec.encode())?;
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
