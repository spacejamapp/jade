//! Chain environment

use anyhow::{Result, anyhow};
use service::{
    EntropyBuffer, OpaqueHash, ServiceId,
    service::{RefineContext, ServiceAccount},
};
use std::collections::BTreeMap;

/// Head of a block
#[derive(Clone, Default)]
pub struct Head {
    /// Hash of the block
    pub hash: OpaqueHash,

    /// Slot of the block
    pub slot: u32,
}

/// Chain environment
#[derive(Clone, Default)]
pub struct Chain {
    /// Best block
    pub best: Head,

    /// Entropy buffer
    pub entropy: EntropyBuffer,

    /// Finalized block
    pub finalized: Head,

    /// Service accounts
    pub accounts: BTreeMap<u32, ServiceAccount>,
}

impl Chain {
    /// Find a service code
    pub fn service(&self, service: ServiceId) -> Result<OpaqueHash> {
        tracing::info!("service: {:?}", service);
        self.accounts
            .get(&service)
            .map(|account| account.info.code)
            .ok_or_else(|| anyhow!("Service not found"))
    }

    /// Get the refine context
    ///
    /// TODO: support prerequisites
    pub fn refine_context(&self) -> RefineContext {
        RefineContext {
            anchor: self.best.hash,
            state_root: Default::default(),
            beefy_root: Default::default(),
            lookup_anchor: self.finalized.hash,
            lookup_anchor_slot: self.finalized.slot,
            prerequisites: Default::default(),
        }
    }
}
