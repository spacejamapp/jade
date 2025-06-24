//! Spacejam Node config

use std::{net::SocketAddr, path::PathBuf};

/// Config for the Spacejam Node
pub struct Node {
    /// The quic transport address
    pub quic: SocketAddr,

    /// the path to the spec file
    pub spec: Option<PathBuf>,

    /// the path to the chain data
    pub data: PathBuf,
}
