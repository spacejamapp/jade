//! JAM Bootstrap Service
//!
//! Use by concatenating one or more encoded `Instruction`s into a work item's payload.

#![cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), no_std)]
#![allow(clippy::unwrap_used)]

extern crate alloc;

use alloc::{format, string::ToString, vec, vec::Vec};
use jam_bootstrap_service_common::{Instruction, ServiceRegistry, SERVICE_REGISTRY_KEY};
use jam_pvm_common::{
    accumulate::*,
    jam_types::*,
    refine::{export_slice, import},
    *,
};

const BENCHMARK_CYCLES: u32 = 1_000;

#[allow(dead_code)]
struct Service;
jam_pvm_common::declare_service!(Service);

impl jam_pvm_common::Service for Service {
    fn refine(
        id: ServiceId,
        payload: WorkPayload,
        _package_hash: WorkPackageHash,
        _context: RefineContext,
        _auth_code_hash: CodeHash,
    ) -> WorkOutput {
        info!(target = "boot", "Bootstrap Service Refine, {id:x}h");

        //let test_hash = [1u8; 32];
        //let mut target = vec![0; 100];
        //let preimage = lookup(test_hash);
        //assert_eq!(&preimage.unwrap(), &[1, 2, 3]);
        let mut cursor = &payload[..];
        let mut out = vec![];
        while !cursor.is_empty() {
            match Instruction::decode(&mut cursor).unwrap() {
                Instruction::Lookup { service, hash } => {
                    info!(target = "boot", "Looking up...");
                    let maybe_data = foreign_lookup(service, &hash);
                    info!(target = "boot", "Got {:?}", maybe_data);
                    if let Some(data) = maybe_data {
                        out.push(Instruction::LookedUp { data });
                    }
                }
                Instruction::RandomStorageRefine(input) => {
                    out.push(Instruction::RandomStorageAccumulate(
                        jam_bootstrap_service_common::test_key_vals::generate_payload(input),
                    ))
                }
                Instruction::Export { data } => {
                    for d in &data {
                        let r = export_slice(d);
                        info!(target = "boot", "Exported slice: {d:?} -> {r:?}");
                    }
                    out.push(Instruction::Exported {
                        count: data.len() as _,
                    })
                }
                Instruction::Import { items } => {
                    let data = items
                        .into_iter()
                        .map(|(index, len)| {
                            import(index as usize)
                                .unwrap()
                                .truncate_into_vec(len as usize)
                        })
                        .collect();
                    out.push(Instruction::Imported { data });
                }
                x => out.push(x),
            };
        }
        debug!(target = "boot", "Returning {:?} into accumulate", out);
        out.encode().into()
    }

    fn accumulate(now: Slot, id: ServiceId, results: Vec<AccumulateItem>) -> Option<Hash> {
        info!(
            target = "boot",
            "Bootstrap Service Accumulate, {id:x}h @{now} ${}",
            my_info().balance
        );
        for raw_instructions in results.into_iter().filter_map(|x| x.result.ok()) {
            for inst in Vec::<Instruction>::decode(&mut &raw_instructions[..]).unwrap() {
                debug!(target = "boot", "Decoded instruction: {:?}", inst);
                match inst {
                    Instruction::CreateService {
                        code_hash,
                        code_len,
                        min_item_gas,
                        min_memo_gas,
                        endowment,
                        memo,
                        registration,
                    } => {
                        let id = create_service(
                            &code_hash,
                            code_len as usize,
                            min_item_gas,
                            min_memo_gas,
                        );
                        if let Ok(id) = id {
                            info!(target = "boot", "Created service {id:x}h");
                            set(b"created", id).expect("balance?");

                            if let Some(code) = lookup(&code_hash) {
                                // Can provide the code right away.
                                let e = provide(id, &code);
                                info!(target = "boot", "Code provision resulted in {e:?}");
                            }

                            info!(target = "boot", "Attempting transfer, gas={}", gas());
                            let e = transfer(id, endowment, min_memo_gas, &memo);
                            info!(
                                target = "boot",
                                "Transfer of {endowment} with {min_memo_gas} gas resulted in {e:?}"
                            );
                            if let Some(registration) = registration {
                                let mut registry: ServiceRegistry =
                                    get(SERVICE_REGISTRY_KEY).unwrap_or_default();
                                registry.update(registration, id, code_hash);
                                set(SERVICE_REGISTRY_KEY, &registry).expect("balance?");
                            }
                        } else {
                            error!("Failed to create!");
                        }
                    }
                    Instruction::Upgrade {
                        code_hash,
                        min_item_gas,
                        min_memo_gas,
                    } => {
                        upgrade(&code_hash, min_item_gas, min_memo_gas);
                        info!(target = "boot", "Upgraded!");
                    }
                    Instruction::Transfer {
                        destination,
                        amount,
                        gas_limit,
                        memo,
                    } => {
                        info!(target = "boot", "Gas remaining: {}", gas());
                        info!(
                            "Attempting transfer: {} {} {} {:?}",
                            destination, amount, gas_limit, memo
                        );
                        let e = transfer(destination, amount, gas_limit, &memo);
                        info!(target = "boot", "Result: {:?}", e);
                        (destination, amount, memo)
                            .using_encoded(|d| set_storage(b"transferred", d).expect("balance?"));
                    }
                    Instruction::Zombify { ejector } => {
                        info!(
                            target = "boot",
                            "Zombifying service. Ejector: #{:x}", ejector
                        );
                        let info = my_info();
                        if info.bytes >= 81 && info.items == 2 {
                            if forget(&info.code_hash, info.bytes as usize - 81).is_ok() {
                                zombify(ejector);
                                info!(target = "boot", "Zombified");
                            } else {
                                error!("Failed to zombify - invalid code_hash?");
                            }
                        } else {
                            error!("Failed to zombify - laggards in storage/lookup?");
                        }
                    }
                    Instruction::Eject { target, code_hash } => {
                        info!(
                            target = "boot",
                            "Ejecting service #{:x} with code_hash {:?}", target, code_hash
                        );
                        let e = eject(target, &code_hash);
                        info!(target = "boot", "Result: {:?}", e);
                    }
                    Instruction::DeleteItems { storage_items } => {
                        let mut fail = 0;
                        for i in &storage_items {
                            info!(target = "boot", "Deleting item: {:?}", i);
                            if remove_storage(i).is_none() {
                                error!("Failed to remove item: {:?}", i);
                                fail += 1;
                            }
                        }
                        info!(
                            "{} items deleted successfully, {} keys not found",
                            storage_items.len() - fail,
                            fail
                        );
                    }
                    Instruction::LookedUp { data } => {
                        set_storage(b"looked_up", &data[..]).expect("balance?");
                    }
                    Instruction::Imported { data } => {
                        info!(
                            target = "boot",
                            "Imported data {:?}",
                            data.iter().cloned().map(AnyVec)
                        );
                        set_storage(b"imported", &(data.len() as u32).encode()).expect("balance?");
                        for (i, d) in data.into_iter().enumerate() {
                            set_storage(alloc::format!("import-{i}").as_bytes(), &d[..])
                                .expect("balance?");
                        }
                    }
                    Instruction::Exported { count } => {
                        info!(target = "boot", "Exported {count} items");
                        set_storage(b"exported", &count.encode()).expect("balance?");
                    }
                    Instruction::Solicit { hash, len } => {
                        solicit(&hash, len as usize).unwrap();
                        info!(target = "boot", "Solicited {hash} of length {len}");
                        set_storage(b"requested", &hash[..]).expect("balance?");
                    }
                    Instruction::Forget { hash, len } => {
                        let q = query(&hash, len as usize).unwrap();
                        info!(
                            target = "boot",
                            "Query result: {:?} (fi: {:?})",
                            q,
                            q.forget_implication(now)
                        );
                        forget(&hash, len as usize).unwrap();
                        set_storage(b"unrequested", &hash[..]).expect("balance?");
                    }
                    Instruction::Assign { core, queue } => {
                        info!(
                            target = "boot",
                            "Assigning core {:?} to queue {:?}", core, queue
                        );
                        match assign(core, &queue) {
                            Ok(_) => info!(target = "boot", "Assigned!"),
                            Err(_) => error!("Failed to assign!"),
                        }
                    }
                    Instruction::Bless {
                        manager: b,
                        assign: a,
                        designate: d,
                        auto_acc,
                    } => {
                        bless(b, a, d, &auto_acc);
                        info!(
                            "Blessed services m: #{}, a: #{}. v: #{}. aa: {:?}",
                            b, a, d, auto_acc
                        );
                    }
                    Instruction::Designate { keys } => {
                        designate(&keys);
                        info!(target = "boot", "Designated keys {:?}", keys);
                    }
                    Instruction::Yield { hash } => {
                        yield_hash(&hash);
                        info!(target = "boot", "Yielded hash {:?}", hash);
                    }
                    Instruction::Provide {
                        service_id,
                        preimage,
                    } => {
                        let r = provide(service_id, &preimage);
                        info!(
                            target = "boot",
                            "Provided preimage to {service_id}: {:?}", r
                        );
                    }
                    Instruction::Checkpoint => {
                        checkpoint();
                        info!(target = "boot", "Checkpointed!");
                    }
                    Instruction::Panic => {
                        panic!("Panic instruction executed!");
                    }
                    Instruction::RandomStorageAccumulate(result_refine) => {
                        if let Ok(keys) = result_refine {
                            let mut count: u64 = get_storage(b"count_random_storage")
                                .map(|v| u64::decode(&mut v.as_slice()).unwrap())
                                .unwrap_or(0);
                            // Warning this instruction is for testing. Most of the work is done in
                            // accumulate which is bad design.
                            for item in keys.items.into_iter() {
                                set_storage(&item.key[..], &item.key[..]).expect("balance?");
                                count += 1;
                            }
                            set_storage(b"count_random_storage", &count.encode()[..])
                                .expect("balance?");
                        }
                    }
                    Instruction::Benchmark => {
                        for i in 0..BENCHMARK_CYCLES {
                            set_storage(
                                format!("item-{now}-{i}").as_bytes(),
                                format!("{i}").as_bytes(),
                            )
                            .expect("balance low");
                            checkpoint();
                        }
                    }
                    i => {
                        info!(target = "boot", "Instruction not handled: {:?}", i);
                    }
                }
            }
        }
        None
    }

    fn on_transfer(_slot: Slot, _id: ServiceId, items: Vec<TransferRecord>) {
        for TransferRecord {
            source,
            amount,
            memo,
            ..
        } in items.into_iter()
        {
            let count = get::<u32>(b"transfer-count").unwrap_or(0);
            set(b"transfer-count", count + 1).expect("balance?");
            info!(
                target = "boot",
                "Received transfer from {source} of {amount} with memo {}",
                alloc::string::String::from_utf8(memo.as_ref().to_vec())
                    .unwrap_or("???".to_string())
            );
            set_storage(
                alloc::format!("transfer{count}").as_bytes(),
                &(source, amount, memo).encode()[..],
            )
            .expect("balance?");
        }
    }
}
