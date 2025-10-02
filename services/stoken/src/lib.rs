#![cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), no_std)]

pub use {instruction::Instruction, storage::Holders};

pub mod instruction;
mod service;
mod storage;

/// The service blob for the simple token service
#[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
pub const SERVICE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/service.jam"));
