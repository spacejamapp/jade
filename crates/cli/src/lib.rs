#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub use {
    clap,
    manifest::{ModuleType, Profile},
    util::build,
};

pub mod builder;
pub mod cmd;
pub mod manifest;
pub mod util;
