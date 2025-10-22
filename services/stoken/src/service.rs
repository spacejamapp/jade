//! Simple Token Service

use crate::{Holders, Instruction};
use jade::{
    error, info,
    prelude::Vec,
    service::{
        OpaqueHash,
        service::WorkExecResult,
        vm::{AccumulateItem, Operand},
    },
};

#[jade::refine]
fn refine(
    _core: u16,
    _index: u16,
    _id: u32,
    payload: Vec<u8>,
    _package_hash: OpaqueHash,
) -> Vec<u8> {
    info!("entering refine logic ...");
    let Ok(instructions) = codec::decode::<Vec<Instruction>>(payload.as_slice()) else {
        error!(
            target = "simple-token-service",
            "failed to decode instructions"
        );
        return Vec::new();
    };

    info!(
        target = "simple-token-service",
        "decoded payload as instructions: {:?}", instructions
    );
    payload
}

#[jade::accumulate]
fn accumulate(_now: u32, _id: u32, items: Vec<AccumulateItem>) -> Option<OpaqueHash> {
    info!("accumulate items: {}", items.len());
    let mut holders = Holders::get();
    for raw_instructions in items.into_iter().filter_map(|x| {
        if let AccumulateItem::Operand(Operand {
            data: WorkExecResult::Ok(data),
            ..
        }) = x
        {
            Some(data)
        } else {
            None
        }
    }) {
        let instructions = codec::decode::<Vec<Instruction>>(&raw_instructions).unwrap();
        jade::info!("instructions: {:?}", instructions);
        for inst in instructions {
            match inst {
                Instruction::Mint { to, amount } => {
                    info!(
                        target = "simple-token-service",
                        "minting {} tokens to {}", amount, to
                    );
                    holders.mint(to, amount);
                }
                Instruction::Transfer { from, to, amount } => {
                    holders.transfer(from, to, amount);
                }
            }
        }
    }

    None
}
