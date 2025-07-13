//! Refine environment

use crate::env::Account;
use podec::{Decode, Encode};
use std::collections::BTreeMap;

/// Refine environment
#[derive(Encode, Decode, Clone)]
pub struct Refine {
    /// The timeslot of the refine
    pub timeslot: u32,

    /// The code of the service
    pub code: Vec<u8>,

    /// (δ) accounts for historical lookup
    pub accounts: BTreeMap<u32, Account>,

    /// (i) The index of the work item
    pub index: u16,

    /// (o) The authorization output
    pub output: Vec<u8>,

    /// (i) The segment imports of the refine
    pub imports: Vec<[u8; 4104]>,

    /// (ς) export segment offset
    pub offset: u16,
}
