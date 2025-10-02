//! Command registry

use clap::{Parser, command};
pub use {build::Build, new::New};

mod build;
mod new;

/// Jam service command line interface
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
            Command::New(new) => new.run(),
        }
    }
}

/// JAM command line interface
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub enum Command {
    #[command(about = "Build a JAM PVM blob")]
    Build(Build),
    #[command(about = "Create a new JAM service")]
    New(New),
}
