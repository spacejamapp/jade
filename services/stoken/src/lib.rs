#![cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), no_std)]

use jade::prelude::OpaqueHash;

pub mod instruction;

#[jade::accumulate]
fn accumulate(_slot: u32, _id: u32, _results: u32) -> Option<OpaqueHash> {
    unimplemented!()
}

#[jade::refine]
fn refine(_core: u16, _index: u16, _id: u32, _payload: Vec<u8>, _package: OpaqueHash) -> Vec<u8> {
    unimplemented!()
}
