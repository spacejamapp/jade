//! Accumulate environment

use crate::env::Account;
use jam_pvm_common::jam_types::AccumulateItem;
use std::collections::BTreeMap;

/// Accumulate environment
pub struct Accumulate {
    /// (U) The context of the accumulate
    pub context: AccumulateContext,

    /// (N_t) The timeslot of the accumulate
    pub timeslot: u32,

    /// (N_s) The service index
    pub service: u32,

    /// (N_g) The gas of the accumulate
    pub gas: u64,

    /// (O) The items of the accumulate
    pub items: Vec<AccumulateItem>,

    /// The entropy of the accumulation
    pub entropy: [u8; 32],
}

/// Accumulate context
pub struct AccumulateContext {
    /// d (δ) The accounts
    pub accounts: BTreeMap<u32, Account>,

    /// i (ι) The upcoming validators
    pub validators: Vec<ValidatorData>,

    /// q (φ) The authorization queue
    pub authorization: [Vec<[u8; 32]>; 2],

    /// χ (χ) The privileged service indices
    pub privileges: Privileges,
}

/// The privileged service indices (χ)
pub struct Privileges {
    /// The bless service id (χm)
    pub bless: u32,

    /// The designate service id (χv)
    pub designate: u32,

    /// The assign service id (χa)
    pub assign: u32,

    /// The always accumulate service ids (χg)
    pub always_acc: BTreeMap<u32, u64>,
}

/// The validator data
pub struct ValidatorData {
    /// The bandersnatch public key
    pub bandersnatch: [u8; 32],

    /// The ed25519 public key
    pub ed25519: [u8; 32],

    /// The bls public key
    pub bls: [u8; 32],

    /// The bls secret key
    pub bls_secret: [u8; 144],

    /// The metadata of the validator
    pub metadata: [u8; 128],
}
