//! Simple Token Service Storage

use alloc::collections::BTreeMap;
use codec::{Decode, Encode};
use jam_pvm_common::{accumulate, error};

/// A map of account IDs to their balances
#[derive(Encode, Decode, Default)]
pub struct Holders {
    inner: BTreeMap<u32, u64>,
}

impl Holders {
    /// Get the holders map
    pub fn get() -> Self {
        accumulate::get(b"holders").unwrap_or_default()
    }

    /// Save the holders map
    pub fn save(&self) {
        accumulate::set(b"holders", &self.encode()).expect("failed to encode holders");
    }

    /// Transfer tokens from one account to another
    pub fn transfer(&mut self, from: u32, to: u32, amount: u64) {
        let from_balance = self.inner.get(&from).copied().unwrap_or(0);
        let to_balance = self.inner.get(&to).copied().unwrap_or(0);

        if from_balance < amount {
            error!("insufficient balance");
            return;
        }

        if to_balance + amount > u64::MAX {
            error!("balance overflow");
            return;
        }

        self.inner.insert(from, from_balance - amount);
        self.inner.insert(to, to_balance + amount);
    }

    /// Mint the given amount of tokens to the given account
    pub fn mint(&mut self, to: u32, amount: u64) {
        let balance = self.inner.get(&to).copied().unwrap_or(0);

        if balance + amount > u64::MAX {
            error!("balance overflow");
            return;
        }

        self.inner.insert(to, balance + amount);
    }
}
