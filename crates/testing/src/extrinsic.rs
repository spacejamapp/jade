//! extrinsic context

use service::OpaqueHash;

/// Extrinsic context
pub struct Extrinsic {
    /// The extrinsic
    pub extrinsic: Vec<u8>,

    /// The extrinsic hash
    pub hash: OpaqueHash,

    /// The extrinsic length
    pub len: u32,
}
