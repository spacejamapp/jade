//! Command registry

use build::Build;
use clap::{Parser, command};

mod build;

/// jam service command line interfaces
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct App {
    #[command(subcommand)]
    command: Command,
}

impl App {
    /// Run the command.
    pub fn run(&self) -> Result<(), anyhow::Error> {
        match &self.command {
            Command::Build(build) => build.run(),
        }
    }
}

/// JAM command line interface
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub enum Command {
    #[command(about = "Build a JAM PVM blob")]
    Build(Build),
}
