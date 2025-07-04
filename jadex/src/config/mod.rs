//! Config for the JAM index

use clap::Parser;
use std::{net::SocketAddr, path::PathBuf};
pub use {
    graphql::{Cors, Graphql},
    spacejam::Builder,
};

mod graphql;

/// Config for the JAM index
pub struct Config {
    /// The node config
    pub node: Builder,

    /// the graphql config
    pub graphql: Graphql,
}
