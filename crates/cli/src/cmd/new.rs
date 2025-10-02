//! `jam new` command

use clap::Parser;
use std::{fs, path::PathBuf, process::Command};

/// CLI utility for creating a new JAM service
#[derive(Parser, Debug, Default)]
pub struct New {
    /// Name of the service to create
    name: String,

    /// Path to create the service
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    /// Do not initialize a git repository
    #[arg(short, long, name = "no-git")]
    nogit: bool,
}

impl New {
    /// Run the command.
    pub fn run(&self) -> anyhow::Result<()> {
        let target = self.path.join(self.name.clone());
        let mut git = Command::new("git");
        git.args([
            "clone",
            "https://github.com/spacejamapp/service-template.git",
            "--depth=1",
        ]);
        git.arg(target.clone());
        git.status()?;

        // remove git files if not desired
        if self.nogit {
            fs::remove_file(target.join(".git")).ok();
            fs::remove_file(target.join(".gitignore")).ok();
            fs::remove_file(target.join(".github")).ok();
        }

        Ok(())
    }
}
