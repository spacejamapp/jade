#![cfg(test)]

use crate::{Holders, Instruction};
use codec::{Decode, Encode};
use testing::Env;

const ALICE: u32 = 300;
const BOB: u32 = 301;

fn holders(env: &Env) -> Holders {
    let encoded = env
        .accounts
        .get(&env.id)
        .expect("failed to get account")
        .storage
        .get(Holders::key())
        .expect("failed to get holders");

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
