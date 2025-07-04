//! Spacejam Node config

use network::Network;
use std::{fs, net::SocketAddr, path::PathBuf, sync::Arc};

/// Spacejam node builder
#[derive(Clone)]
pub struct Node {
    /// The genesis path
    pub chain: Option<PathBuf>,

    /// The data path
    pub data_path: String,

    /// Whether running in dev mode
    pub dev: bool,

    /// Whether running in light mode
    pub light: bool,

    /// The network configuration
    pub network: network::Config,

    /// The RPC address
    pub rpc: SocketAddr,

    /// The seed of the validator
    pub validator: Option<String>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            chain: None,
            data_path: default::data_path(),
            rpc: SocketAddr::from(([0, 0, 0, 0], 6789)),
            network: network::Config::default(),
            validator: None,
            dev: false,
            light: false,
        }
    }
}

mod default {
    /// The default data path
    pub fn data_path() -> String {
        dirs::data_dir()
            .unwrap_or_default()
            .join("spacejam")
            .to_string_lossy()
            .to_string()
    }
}
