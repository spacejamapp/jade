//! Work package builder implementation

use crate::Jam;
use anyhow::Result;
use service::{
    ServiceId,
    service::{WorkItem, WorkPackage},
};

impl Jam {
    /// Add a work item
    ///
    /// TODO: validate the work item
    pub fn add_item(&mut self, item: WorkItem) {
        self.items.push(item);
    }

    /// Build a work package
    pub fn build(&mut self) -> Result<WorkPackage> {
        let package = WorkPackage {
            authorization: self.auth.token.clone(),
            auth_code_host: self.auth.host,
            auth_code_hash: self.auth.code_hash,
            config: self.auth.config.clone(),
            context: self.chain.refine_context(),
            items: self.items.drain(..).collect(),
        };

        Ok(package)
    }

    /// pack a work item
    pub fn pack(&mut self, service: ServiceId, payload: Vec<u8>) -> Result<()> {
        let item = WorkItem {
            service,
            code_hash: self.chain.service(service)?,
            payload,
            refine_gas_limit: 1_000_000,
            accumulate_gas_limit: 1_000_000,
            import_segments: Default::default(),
            extrinsic: Default::default(),
            export_count: Default::default(),
        };

        self.add_item(item);
        Ok(())
    }

    /// Send a work package
    pub fn send(&mut self, service: ServiceId, payload: Vec<u8>) -> Result<WorkPackage> {
        self.pack(service, payload)?;
        self.build()
    }
}
