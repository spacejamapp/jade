//! Environment for service testing

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
mod refine;

/// The execution environment
pub struct Env {
    /// The accounts of the environment
    pub accounts: BTreeMap<u32, Account>,

    /// The code we are about to execute
    pub code: Vec<u8>,
}
