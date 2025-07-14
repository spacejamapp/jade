//! Environment for service testing

use crate::crypto;
use jam_types::WorkPackage;
use podec::{Decode, Encode};
use std::collections::BTreeMap;
pub use {
    account::Account,
    accumulate::{Accumulate, AccumulateContext, Privileges, ValidatorData},
    authorize::Authorize,
    refine::Refine,
    report::WorkResult,
};

mod account;
mod accumulate;
mod authorize;
mod loader;
mod package;
mod refine;
mod report;

/// The execution environment
#[derive(Encode, Decode, Clone, Default)]
pub struct Env {
    /// The accounts of the environment
    pub accounts: BTreeMap<u32, Account>,

    /// The authorize environment
    pub authorize: Authorize,

    /// The id of the service we are about to execute
    pub id: u32,

    /// The work package we are about to execute
    pub package: WorkPackage,

    /// The results of the work items which will be used
    /// in the accumulation step.
    pub result: Vec<WorkResult>,

    /// The timeslot we are about to execute
    pub timeslot: u32,

    /// The validators of the environment
    pub validators: Vec<ValidatorData>,
}

impl Env {
    /// Add an account to the environment
    pub fn add_account(&mut self, id: u32, code: Vec<u8>) -> u32 {
        let hash = crypto::blake2b(&code);
        let mut account = Account {
            code: hash,
            storage: BTreeMap::new(),
            preimage: BTreeMap::new(),
            lookup: BTreeMap::new(),
            balance: 100_000_000,
            accumulate_gas: 100_000,
            transfer_gas: 100_000,
        };
        account.preimage.insert(hash, code);
        self.accounts.insert(id, account);
        id
    }
}
