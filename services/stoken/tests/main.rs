//! Basic VM tests

use jade::testing::Jam;
use stoken::{Holders, Instruction, SERVICE};

const AUTHORIZER_ID: u32 = 500;
const SERVICE_ID: u32 = 501;
const ALICE: u32 = 0;

#[test]
fn test_mint() {
    jade::testing::util::init_logger();

    // Set up JAM with authorization using the null authorizer service
    let mut jam = Jam::default().with_auth(AUTHORIZER_ID, nauth::SERVICE.to_vec());
    jam.add_service(SERVICE_ID, SERVICE.to_vec());

    // 1. send a mint instruction
    let amount = 100;
    let instr = vec![Instruction::Mint { to: ALICE, amount }];
    let info = jam
        .execute(SERVICE_ID, codec::encode(&instr).unwrap())
        .expect("failed to execute work item");

    // 2. check the balance
    let holders: Holders = info
        .get_storage(SERVICE_ID, Holders::key())
        .expect("failed to get holders");
    assert_eq!(holders.balance(ALICE), amount);
}
