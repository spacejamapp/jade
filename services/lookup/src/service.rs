//! Lookup service implementation.

use crate::{
    Instruction,
    storage::{LookupStore, LookupTarget},
};
use jade::{
    error,
    host::storage,
    info,
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
    if let Ok(instructions) = codec::decode::<Vec<Instruction>>(payload.as_slice()) {
        info!(
            target = "lookup-service",
            "refine payload decoded into {} instructions",
            instructions.len()
        );
        payload
    } else {
        error!(target = "lookup-service", "failed to decode instructions");
        Vec::new()
    }
}

#[jade::accumulate]
fn accumulate(_now: u32, id: u32, items: Vec<AccumulateItem>) -> Option<OpaqueHash> {
    let mut store = LookupStore::get();
    let mut updated = false;
    let service_id = u64::from(id);

    for raw in items.into_iter().filter_map(|item| {
        if let AccumulateItem::Operand(Operand {
            data: WorkExecResult::Ok(data),
            ..
        }) = item
        {
            Some(data)
        } else {
            None
        }
    }) {
        let Ok(instructions) = codec::decode::<Vec<Instruction>>(&raw) else {
            error!(
                target = "lookup-service",
                "failed to decode instructions during accumulate"
            );
            continue;
        };

        for instruction in instructions {
            match instruction {
                Instruction::Lookup { hash } => {
                    let target = LookupTarget {
                        service: service_id,
                        hash,
                    };
                    if store.contains(&target) {
                        continue;
                    }

                    match storage::lookup(hash) {
                        Some(preimage) => {
                            info!(
                                target = "lookup-service",
                                "cached preimage from current service"
                            );
                            store.put(target, preimage);
                            updated = true;
                        }
                        None => error!(
                            target = "lookup-service",
                            "preimage not found in current service storage"
                        ),
                    }
                }
                Instruction::LookupFrom { service, hash } => {
                    let target = LookupTarget { service, hash };
                    if store.contains(&target) {
                        continue;
                    }

                    match storage::lookup_at(service, hash) {
                        Some(preimage) => {
                            info!(
                                target = "lookup-service",
                                "cached preimage from service {}", service
                            );
                            store.put(target, preimage);
                            updated = true;
                        }
                        None => error!(
                            target = "lookup-service",
                            "preimage not found in service {}", service
                        ),
                    }
                }
            }
        }
    }

    if updated {
        store.save();
    }

    None
}
