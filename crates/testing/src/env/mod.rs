//! Environment for service testing

use crate::{crypto, Simulator};
use anyhow::Result;
use jam_types::WorkPackage;
use podec::{Decode, Encode};
use std::collections::BTreeMap;
pub use {
    account::Account,
    accumulate::{Accumulate, AccumulateContext, Privileges, ValidatorData},
    authorize::Authorize,
    logs::Logs,
    refine::Refine,
    report::WorkResult,
};

mod account;
mod accumulate;
mod authorize;
mod loader;
mod logs;
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

    /// The logs of the execution
    pub logs: Logs,

    /// The target to filter the logs
    pub target: String,
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
            accumulate_gas: 1_000_000,
            transfer_gas: 1_000_000,
        };
        account.lookup.insert((hash, code.len() as u32), vec![0]);
        account.preimage.insert(hash, code);
        self.accounts.insert(id, account);
        id
    }

    /// Execute the is_authorized step
    pub fn is_authorized(&mut self) -> Result<()> {
        let simulator = Simulator::default();
        let result = simulator.is_authorized(self)?;
        *self = result.env;
        self.logs.is_authorized = result.logs;
        Ok(())
    }

    /// Execute the refine step
    pub fn refine(&mut self) -> Result<()> {
        let simulator = Simulator::default();
        let result = simulator.refine(self)?;
        *self = result.env;
        self.logs.refine = result.logs;
        Ok(())
    }

    /// Execute the accumulate step
    pub fn accumulate(&mut self) -> Result<()> {
        let simulator = Simulator::default();
        let result = simulator.accumulate(self)?;
        *self = result.env;
        self.logs.accumulate = result.logs;
        Ok(())
    }

    /// Execute the work package
    pub fn transact(&mut self) -> Result<()> {
        // TODO: is_authorized
        // self.is_authorized()?;

        // TODO: refine
        // self.refine()?;
        // for work in self.result.iter() {
        //     if let Err(e) = &work.result {
        //         anyhow::bail!("failed to process refine: {e:?}");
        //     }
        // }

        // handle accumulation only atm
        self.accumulate()?;

        println!("accumulate logs: {:#?}", self.logs.accumulate);
        Ok(())
    }
}
