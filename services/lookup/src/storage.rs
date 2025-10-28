//! Storage helpers for the lookup service.

use jade::{
    error,
    host::storage,
    prelude::{BTreeMap, OpaqueHash, Vec},
};
use serde::{Deserialize, Serialize};

/// Identifier for a stored preimage.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LookupTarget {
    /// Service identifier where the preimage resides.
    pub service: u64,
    /// Hash of the preimage.
    pub hash: OpaqueHash,
}

/// Collection of cached preimages.
#[derive(Serialize, Deserialize, Default)]
pub struct LookupStore {
    entries: BTreeMap<LookupTarget, Vec<u8>>,
}

impl LookupStore {
    /// Load the stored entries from persistent storage.
    pub fn get() -> Self {
        storage::read(Self::key()).unwrap_or_default()
    }

    /// Persist the store.
    pub fn save(&self) {
        if let Err(err) = storage::write(Self::key(), self) {
            error!(
                target = "lookup-service",
                "failed to save lookup store: {:?}", err
            );
        }
    }

    /// Insert or replace a cached preimage.
    pub fn put(&mut self, target: LookupTarget, preimage: Vec<u8>) {
        self.entries.insert(target, preimage);
    }

    /// Retrieve a cached preimage.
    pub fn get_entry(&self, target: &LookupTarget) -> Option<&[u8]> {
        self.entries.get(target).map(Vec::as_slice)
    }

    /// Check if an entry exists.
    pub fn contains(&self, target: &LookupTarget) -> bool {
        self.entries.contains_key(target)
    }

    /// Enumerate all cached entries.
    pub fn entries(&self) -> &BTreeMap<LookupTarget, Vec<u8>> {
        &self.entries
    }

    /// Storage key for the lookup store.
    pub const fn key() -> &'static [u8] {
        b"lookup::store"
    }
}
