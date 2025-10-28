//! Instructions for the lookup service.

use jade::prelude::OpaqueHash;
use serde::{Deserialize, Serialize};

/// Commands that the lookup service can execute.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    /// Lookup a preimage stored under the current service account.
    Lookup { hash: OpaqueHash },
    /// Lookup a preimage stored under the specified service account.
    LookupFrom { service: u64, hash: OpaqueHash },
}
