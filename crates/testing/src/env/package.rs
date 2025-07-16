//! Work package APIs

use crate::env::{Env, WorkResult};
use anyhow::Result;
use jam_types::WorkItem;

impl Env {
    /// Pack an execution item into the current work package
    pub fn send_legacy(&mut self, payload: Vec<u8>) -> Result<()> {
        let item = WorkItem {
            service: self.id,
            code_hash: self.accounts[&self.id].code.into(),
            payload: payload.into(),
            refine_gas_limit: 1_000_000,
            accumulate_gas_limit: 1_000_000,
            import_segments: Default::default(),
            extrinsics: vec![],
            export_count: 0,
        };

        self.package
            .items
            .try_push(item)
            .map_err(|_| anyhow::anyhow!("Failed to add work item: bounded vector is full"))?;
        Ok(())
    }

    /// Pack an execution item into the current work package
    ///
    /// NOTE: the instructions will be composed as the work results
    /// directly atm
    pub fn send(&mut self, payload: Vec<u8>) -> Result<()> {
        let item = WorkResult {
            service_id: self.id,
            code_hash: self.accounts[&self.id].code.into(),
            payload_hash: Default::default(),
            accumulate_gas: 1_000_000,
            result: Ok(payload),
            refine_load: Default::default(),
        };

        self.result.push(item);
        Ok(())
    }
}
