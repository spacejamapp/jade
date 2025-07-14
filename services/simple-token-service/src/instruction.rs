//! Simple Token Service Instructions

use codec::{Decode, Encode};

#[derive(Encode, Decode, Debug)]
pub enum Instruction {
    /// Mint tokens to the given account
    Mint { to: u32, amount: u64 },
    /// Transfer tokens from one account to another
    Transfer { from: u32, to: u32, amount: u64 },
}
