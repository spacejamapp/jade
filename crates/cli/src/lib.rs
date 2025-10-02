//! Command line interface for JAM services

pub use {
    clap,
    manifest::{ModuleType, Profile},
    util::build,
};

pub mod builder;
pub mod cmd;
pub mod manifest;
pub mod util;
