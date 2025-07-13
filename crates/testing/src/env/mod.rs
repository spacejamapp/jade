//! Environment for service testing

use podec::{Decode, Encode};
use std::collections::BTreeMap;
pub use {
    account::Account,
    accumulate::{Accumulate, AccumulateContext, Privileges, ValidatorData},
    authorize::Authorize,
    refine::Refine,
};

mod account;
mod accumulate;
mod authorize;
mod loader;
mod package;
mod refine;

/// The execution environment
#[derive(Encode, Decode, Clone)]
pub struct Env {
    /// The accounts of the environment
    pub accounts: BTreeMap<u32, Account>,

    /// The code we are about to execute
    pub code: Vec<u8>,

    /// The accumulate environment
    pub accumulate: Accumulate,

    /// The authorize environment
    pub authorize: Authorize,

    /// The refine environment
    pub refine: Refine,
}
