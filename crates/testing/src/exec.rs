//! Execution API of JAM VM

use crate::{Jam, key};
use anyhow::Result;
use service::{
    ServiceId,
    api::{
        AccumulateArgs, AccumulateState, Accumulated, AuthorizeArgs, Reason, RefineArgs,
        ValidatorData,
    },
    service::{
        Privileges, RefineLoad, ServiceAccount, WorkExecResult, WorkPackage, WorkResult,
        result::Executed,
    },
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
        service: ServiceId,
        key: &[u8],
    ) -> Option<V> {
        let account = self.accounts.get(&service)?;
        let vkey = key.to_vec();
        let key = key::storage(service, &vkey);
        let encoded = account.storage.get(key.as_ref())?;
        codec::decode(&mut &encoded[..]).ok()
    }
}

impl Jam {
    /// Execute a work item directly
    ///
    /// TODO: introduce better execution result
    pub fn execute(&mut self, service: ServiceId, payload: Vec<u8>) -> Result<ExecutionInfo> {
        let package = self.send(service, payload)?;
        let result = self.refine(&package)?;
        Ok(ExecutionInfo::new(self.accumulate(result)?))
    }

    /// Authorize the work package
    #[tracing::instrument(name = "authorize", skip_all)]
    pub fn authorize(&mut self, work: &WorkPackage, core_idx: u16) -> Result<Executed> {
        tracing::debug!(
            "service={}, code=0x{}",
            work.auth_code_host,
            hex::encode(work.auth_code_hash)
        );

        spacevm::authorize(AuthorizeArgs {
            package: work.clone(),
            core_idx: core_idx,
            accounts: self.chain.accounts.clone(),
            timeslot: self.chain.best.slot,
        })
    }

    /// Refine the work package
    ///
    /// NOTE: run refine for all work items
    #[tracing::instrument(name = "refine", skip_all)]
    pub fn refine(&mut self, work: &WorkPackage) -> Result<Vec<WorkResult>> {
        tracing::debug!("package: items={}", work.items.len());
        if work.items.is_empty() {
            anyhow::bail!("no work items");
        }

        let mut result = Vec::new();
        for (index, item) in work.items.iter().enumerate() {
            let refined = spacevm::refine(RefineArgs {
                accounts: self.chain.accounts.clone(),
                core: 0,
                index: 0,
                package: work.clone(),
                export_offset: 0,
                timeslot: self.chain.best.slot,
                auth_output: Default::default(),
                all_imports: Default::default(),
            })?;

            if !matches!(refined.executed.exec, WorkExecResult::Ok(_)) {
                return Err(anyhow::anyhow!(
                    "work item {index} refine failed: {:?}",
                    refined.executed.exec
                ));
            }

            result.push(WorkResult {
                service_id: item.service,
                code_hash: item.code_hash,
                payload_hash: Default::default(),
                accumulate_gas: Default::default(),
                result: refined.executed.exec,
                refine_load: RefineLoad {
                    gas_used: refined.executed.gas,
                    imports: Default::default(),
                    extrinsic_count: Default::default(),
                    extrinsic_size: Default::default(),
                    exports: Default::default(),
                },
            });
        }

        Ok(result)
    }

    /// Accumulate the work package
    ///
    /// 1. convert work package to work report
    /// 2. run accumulate for all work items
    /// 3. return the accumulated result
    #[tracing::instrument(name = "accumulate", skip_all)]
    pub fn accumulate(&mut self, results: Vec<WorkResult>) -> Result<Vec<Accumulated>> {
        tracing::debug!("work: items={}", results.len());
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
            gas: 1_000_000_000,
            operands,
        })?;

        if !matches!(accumulated.reason, Reason::Halt) {
            anyhow::bail!("accumulate failed: {:?}", accumulated.reason);
        }
        state = accumulated.context.clone();
        self.chain.accounts = state.accounts;
        Ok(vec![accumulated])
    }
}
