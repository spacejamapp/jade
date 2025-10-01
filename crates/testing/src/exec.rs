//! Execution API of JAM VM

use crate::Jam;
use anyhow::Result;
use service::{
    ServiceId,
    api::{AccumulateArgs, AccumulateState, Accumulated, AuthorizeArgs, Reason, ValidatorData},
    service::{Privileges, ServiceAccount, WorkPackage, WorkResult, result::Executed},
    vm::Operand,
};
use std::collections::BTreeMap;

/// The result of an execution
#[derive(Debug, Default)]
pub struct ExecutionInfo {
    /// The refine gas used
    pub refine_gas: u64,

    /// The accumulate gas used
    pub accumulate_gas: u64,

    /// The account changes
    pub accounts: BTreeMap<ServiceId, ServiceAccount>,
}

impl ExecutionInfo {
    /// Create a new execution info
    pub fn new(acc: Vec<Accumulated>) -> Self {
        let mut info = Self::default();
        for acc in acc.iter() {
            info.accumulate_gas += acc.gas;

            // FIXME: need to merge account data
            info.accounts = acc.context.accounts.clone();
        }

        info
    }

    /// Get a storage of an account
    pub fn get_storage<V: serde::de::DeserializeOwned>(
        &self,
        _service: ServiceId,
        _key: &[u8],
    ) -> Option<V> {
        /* let account = self.accounts.get(&service)?;
        let key = account::storage(service, &key.encode());
        let encoded = account.storage.get(key.as_ref())?;
        V::decode(&mut &encoded[..]).ok() */
        None
    }
}

impl Jam {
    /// Execute a work item directly
    ///
    /// TODO: introduce better execution result
    pub fn execute(&mut self, _service: ServiceId, _payload: Vec<u8>) -> Result<ExecutionInfo> {
        /* let package = self.send(service, payload)?;
        let report = self.refine(&package)?;
        Ok(ExecutionInfo::new(self.accumulate(&report)?)) */
        unimplemented!("abccb");
    }

    /// Authorize the work package
    pub fn authorize(&mut self, work: &WorkPackage, core_idx: u16) -> Result<Executed> {
        spacevm::authorize(AuthorizeArgs {
            package: work.clone(),
            core_idx: core_idx,
            accounts: self.chain.accounts.clone(),
            timeslot: self.chain.best.slot,
        })
    }

    /*   /// Refine the work package
    ///
    /// NOTE: run refine for all work items
    pub fn refine(&mut self, work: &WorkPackage) -> Result<WorkReport> {
        let guarantor = InMemoryDataLake::default();
        let (report, _) =
            guarantor.compute_sync::<_, Interpreter>(0, vec![], work, &mut self.chain.accounts)?;

        // verify the work results
        for (index, result) in report.results.iter().enumerate() {
            if !matches!(result.result, WorkExecResult::Ok(_)) {
                return Err(anyhow::anyhow!(
                    "work item {index} refine failed: {:?}",
                    result.result
                ));
            }
        }
        Ok(report)
    } */

    /// Accumulate the work package
    ///
    /// 1. convert work package to work report
    /// 2. run accumulate for all work items
    /// 3. return the accumulated result
    pub fn accumulate(&mut self, results: Vec<WorkResult>) -> Result<Accumulated> {
        if results.is_empty() {
            anyhow::bail!("no results");
        }

        let accounts = self.chain.accounts.clone();
        let mut state = AccumulateState {
            accounts,
            validators: [ValidatorData {
                bandersnatch: Default::default(),
                ed25519: Default::default(),
                bls: [0; 144],
                metadata: [0; 128],
            }; 6],
            authorization: Default::default(),
            privileges: Privileges::default(),
            entropy: Default::default(),
        };

        let service = results.first().expect("checked").service_id;
        let operands = {
            let mut operands = vec![];
            for work in results.iter() {
                operands.push(Operand {
                    package: Default::default(),
                    exports_root: Default::default(),
                    authorizer_hash: Default::default(),
                    auth_output: Default::default(),
                    payload: work.payload_hash,
                    gas: work.accumulate_gas,
                    data: work.result.clone(),
                });
            }
            operands
        };

        // run accumulation
        let accumulated = spacevm::accumulate(AccumulateArgs {
            context: state,
            timeslot: self.chain.best.slot,
            service,
            gas: 0,
            operands,
        })?;

        if matches!(accumulated.reason, Reason::Halt) {
            anyhow::bail!("accumulate failed: {:?}", accumulated.reason);
        }
        state = accumulated.context.clone();
        self.chain.accounts = state.accounts;
        Ok(accumulated)
    }
}
