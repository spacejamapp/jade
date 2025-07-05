//! Service Account

use podec::{Decode, Encode};
use std::collections::BTreeMap;

/// A service account
#[derive(Debug, PartialEq, Eq, Clone, Default, Encode, Decode)]
pub struct Account {
    /// Storage of the account (s)
    pub storage: BTreeMap<Vec<u8>, Vec<u8>>,

    /// The preimage of the service account (p)
    pub preimage: BTreeMap<[u8; 32], Vec<u8>>,

    /// Preimage lookup dictionary (l)
    pub lookup: BTreeMap<([u8; 32], u32), Vec<u32>>,

    /// The code hash of the service account (c)
    pub code: [u8; 32],

    /// The balance of the service account (b)
    pub balance: u64,

    /// The minimum gas in order to execute the accumulate
    /// entry-point of the service code (g)
    pub accumulate_gas: u64,

    /// The minimum required for the on transfer entry-point (m)
    pub transfer_gas: u64,
}
