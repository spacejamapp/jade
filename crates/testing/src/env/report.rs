//! Work report interface

use jam_pvm_common::jam_types::WorkError;
use podec::{Decode, Encode};

/// Represents the result of a work item.
#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode)]
pub struct WorkResult {
    /// The service id
    pub service_id: u32,

    /// The code hash
    pub code_hash: [u8; 32],

    /// The payload hash
    pub payload_hash: [u8; 32],

    /// The accumulate gas
    pub accumulate_gas: u64,

    /// The result of the work item
    pub result: Result<Vec<u8>, WorkError>,

    /// The refine load
    pub refine_load: RefineLoad,
}

/// Represents the load of a refine operation.
#[derive(Debug, PartialEq, Eq, Clone, Default, Encode, Decode)]
pub struct RefineLoad {
    /// The gas used
    pub gas_used: u64,

    /// The number of imports
    pub imports: u16,

    /// The number of extrinsics
    pub extrinsic_count: u16,

    /// The size of the extrinsics
    pub extrinsic_size: u32,

    /// The number of exports
    pub exports: u16,
}
