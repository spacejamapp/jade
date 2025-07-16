#![cfg(test)]

use crate::{Holders, Instruction};
use codec::{Decode, Encode};
use testing::{ext, Env};

const ALICE: u32 = 300;
const BOB: u32 = 301;

fn holders(env: &Env) -> Holders {
    let key = ext::storage_key(env.id, &Holders::key().encode());
    let encoded = env
        .accounts
        .get(&env.id)
        .expect("failed to get account")
        .storage
        .get(&key.to_vec())
        .expect("holders not initialized");
    let encoded = Vec::<u8>::decode(&mut encoded.as_slice()).expect("failed to decode encoded");

    Holders::decode(&mut encoded.as_slice()).expect("failed to decode holders")
}

#[test]
fn test_mint() {
    let mut jam = Env::load().expect("failed to load service environment");
    let amount = 100_000;
    let mint = Instruction::Mint { to: ALICE, amount };

    jam.send(vec![mint].encode())
        .expect("failed to add work item");
    jam.transact().expect("failed to transact changes");

    let holders = self::holders(&jam);
    assert_eq!(holders.balance(ALICE), amount);
}

#[test]
fn test_transfer() {
    let mut jam = Env::load().expect("failed to load service environment");
    let amount = 100_000;
    let half = amount / 2;
    let mint = Instruction::Mint { to: ALICE, amount };
    let transfer = Instruction::Transfer {
        from: ALICE,
        to: BOB,
        amount: half,
    };

    jam.send(vec![mint, transfer].encode())
        .expect("failed to add work item");
    jam.transact().expect("failed to transact changes");

    let holders = self::holders(&jam);
    assert_eq!(holders.balance(ALICE), half);
    assert_eq!(holders.balance(BOB), half);
}
