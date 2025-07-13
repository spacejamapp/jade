//! Environment for service testing

use jam_types::WorkPackage;
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

    /// The hash of the code we are about to execute
    pub hash: [u8; 32],

    /// The id of the service we are about to execute
    pub id: u32,

    /// The work package we are about to execute
    pub package: WorkPackage,

    /// The authorize environment
    pub authorize: Authorize,
}
