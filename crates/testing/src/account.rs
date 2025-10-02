//! Service account builder

use crate::{Jam, key};
use service::{OpaqueHash, ServiceId, service::ServiceAccount};

impl Jam {
    /// Add a service account
    pub fn add_account(&mut self, service: ServiceId, mut account: ServiceAccount) {
        account.index = service;
        self.chain.accounts.insert(service, account);
    }

    /// Add a service account
    pub fn add_service(&mut self, service: ServiceId, code: Vec<u8>) {
        let hash = self.add_preimage(service, code);
        self.set_code(service, hash);
        self.mint(service, 1_000_000_000);
    }

    /// Add a preimage to the service account
    pub fn add_preimage(&mut self, service: ServiceId, preimage: Vec<u8>) -> OpaqueHash {
        let account = self.chain.accounts.entry(service).or_default();
        let hash = service::blake2b(preimage.as_slice());
        let len = preimage.len() as u32;
        account.index = service;
        account.preimage.insert(hash, preimage);
        account
            .lookup
            .insert((hash, len), vec![self.chain.best.slot]);
        hash
    }

    /// Get a storage of an account
    pub fn get_storage<V: serde::de::DeserializeOwned>(
        &self,
        service: ServiceId,
        key: &[u8],
    ) -> Option<V> {
        let account = self.chain.accounts.get(&service)?;
        let vkey = key.to_vec();
        let key = key::storage(service, &codec::encode(&vkey).ok()?);
        let encoded = account.storage.get(key.as_ref())?;
        codec::decode(&encoded[..]).ok()
    }

    /// Set the code of the service account
    pub fn set_code(&mut self, service: ServiceId, code: OpaqueHash) {
        let account = self.chain.accounts.entry(service).or_default();
        account.info.code = code;
    }

    /// Mint balance to a service account
    pub fn mint(&mut self, service: ServiceId, amount: u64) {
        let account = self.chain.accounts.entry(service).or_default();
        account.info.balance += amount;
    }

    /// Add a service account
    pub fn with_account(mut self, service: ServiceId, account: ServiceAccount) -> Self {
        self.chain.accounts.insert(service, account);
        self
    }
}
