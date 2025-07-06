//! Config for the JAM index

use clap::Parser;
use std::{net::SocketAddr, path::PathBuf};
pub use {
    graphql::{Cors, Graphql},
    spacejam::Builder as Node,
};

mod graphql;

/// Config for the JAM index
pub struct Config {
    /// The node config
    pub node: Node,

    /// the graphql config
    pub graphql: Graphql,
}
