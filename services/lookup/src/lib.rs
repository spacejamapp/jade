#![cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), no_std)]

pub use instruction::Instruction;
pub use storage::{LookupStore, LookupTarget};

pub mod instruction;
mod service;
pub mod storage;

/// The service blob for the lookup service.
#[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
pub const SERVICE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/service.jam"));
