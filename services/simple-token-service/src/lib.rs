//! JAM Bootstrap Service
//!
//! Use by concatenating one or more encoded `Instruction`s into a work item's payload.

#![cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), no_std)]
#![allow(clippy::unwrap_used)]

extern crate alloc;

mod instruction;
mod service;
mod storage;

pub use {instruction::Instruction, service::Service, storage::Holders};
