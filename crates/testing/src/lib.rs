#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub use service::service::ServiceAccount as Account;
use service::service::WorkItem;
pub use {auth::Auth, chain::Chain, extrinsic::Extrinsic};

mod account;
mod auth;
mod builder;
mod chain;
mod exec;
mod extrinsic;
pub mod key;
pub mod util;

/// JAM environment
#[derive(Default)]
pub struct Jam {
    /// Chain environment
    chain: Chain,

    /// authorization token
    auth: Auth,

    /// work items
    items: Vec<WorkItem>,

    /// extrinsics
    _extrinsic: Vec<Extrinsic>,
}
