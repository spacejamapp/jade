//! Work package APIs

use crate::env::Env;
use anyhow::Result;
use jam_types::WorkItem;

impl Env {
    /// Pack an execution item into the current work package
    pub fn send(&mut self, payload: Vec<u8>) -> Result<()> {
        let item = WorkItem {
            service: self.id,
            code_hash: self.hash.into(),
            payload: payload.into(),
            refine_gas_limit: 0,
            accumulate_gas_limit: 0,
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
}
