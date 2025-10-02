//! `jam new` command

use clap::Parser;
use std::{path::PathBuf, process::Command};

/// CLI utility for creating a new JAM service
#[derive(Parser, Debug, Default)]
pub struct New {
    /// Name of the service to create
    name: String,

    /// Path to create the service
    #[arg(short, long, default_value = ".")]
    path: PathBuf,
}

impl New {
    /// Run the command.
    pub fn run(&self) -> anyhow::Result<()> {
        let mut git = Command::new("git");
        git.args([
            "clone",
            "https://github.com/spacejamapp/service-template.git",
        ]);
        git.arg(self.path.join(self.name.clone()));
        git.status()?;
        Ok(())
    }
}
