//! Simple Token Service

use crate::{Holders, Instruction};
use alloc::vec::Vec;
use jam_pvm_common::{error, info, jam_types::*};

#[allow(dead_code)]
pub struct Service;
jam_pvm_common::declare_service!(Service);

impl jam_pvm_common::Service for Service {
    fn refine(
        _id: ServiceId,
        payload: WorkPayload,
        _package_hash: WorkPackageHash,
        _context: RefineContext,
        _auth_code_hash: CodeHash,
    ) -> WorkOutput {
        info!("entering refine logic ...");
        let Ok(instructions) = Vec::<Instruction>::decode(&mut payload.0.as_slice()) else {
            error!(
                target = "simple-token-service",
                "failed to decode instructions"
            );
            panic!("failed to decode instructions");
        };

        info!(
            target = "simple-token-service",
            "instructions: {:?}", instructions
        );
        WorkOutput(payload.0)
    }

    fn accumulate(_now: Slot, _id: ServiceId, results: Vec<AccumulateItem>) -> Option<Hash> {
        info!("entering accumulate logic ...");
        let mut holders = Holders::get();
        for raw_instructions in results.into_iter().filter_map(|x| x.result.ok()) {
            for inst in Vec::<Instruction>::decode(&mut &raw_instructions[..]).unwrap() {
                info!(target = "simple-token-service", "instruction: {:?}", inst);
                match inst {
                    Instruction::Mint { to, amount } => {
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

    fn on_transfer(_slot: Slot, _id: ServiceId, _items: Vec<TransferRecord>) {}
}
