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
        accumulate::get(Self::key()).unwrap_or_default()
    }

    /// Save the holders map
    pub fn save(&self) {
        accumulate::set(Self::key(), self.encode()).expect("failed to encode holders");
    }

    /// Get the balance of the given account
    pub fn balance(&self, account: u32) -> u64 {
        self.inner.get(&account).copied().unwrap_or(0)
    }

    /// Get the key of the holders map
    pub const fn key() -> &'static [u8] {
        b"holders"
    }

    /// Transfer tokens from one account to another
    pub fn transfer(&mut self, from: u32, to: u32, amount: u64) {
        let from_balance = self.inner.get(&from).copied().unwrap_or(0);
        let to_balance = self.inner.get(&to).copied().unwrap_or(0);

        if from_balance < amount {
            error!("insufficient balance");
            return;
        }

        self.inner.insert(
            from,
            from_balance.checked_sub(amount).expect("balance overflow"),
        );
        self.inner.insert(
            to,
            to_balance.checked_add(amount).expect("balance overflow"),
        );
        self.save();
    }

    /// Mint the given amount of tokens to the given account
    pub fn mint(&mut self, to: u32, amount: u64) {
        let balance = self.inner.get(&to).copied().unwrap_or(0);
        self.inner
            .insert(to, balance.checked_add(amount).expect("balance overflow"));
        self.save();
    }
}
