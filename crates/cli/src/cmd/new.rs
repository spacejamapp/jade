//! `jam new` command

use clap::Parser;

/// CLI utility for creating a new JAM service
#[derive(Parser, Debug, Default)]
pub struct New {
    /// Name of the service to create
    #[arg(short, long)]
    name: String,
}
