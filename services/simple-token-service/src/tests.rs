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
    let mint = Instruction::Mint {
        to: ALICE,
        amount: 1000_000,
    };

    jam.send(vec![mint].encode())
        .expect("failed to add work item");
}
