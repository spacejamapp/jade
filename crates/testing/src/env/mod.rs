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
#[derive(Encode, Decode)]
pub struct Env {
    /// The accounts of the environment
    accounts: BTreeMap<u32, Account>,

    /// The code we are about to execute
    code: Vec<u8>,

    /// The accumulate environment
    accumulate: Accumulate,

    /// The authorize environment
    authorize: Authorize,

    /// The refine environment
    refine: Refine,
}
