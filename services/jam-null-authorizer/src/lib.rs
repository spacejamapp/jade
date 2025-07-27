#![cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), no_std)]

extern crate alloc;
use jam_pvm_common::{is_authorized::*, *};
use jam_types::*;

#[allow(dead_code)]
struct Authorizer;
jam_pvm_common::declare_authorizer!(Authorizer);

impl jam_pvm_common::Authorizer for Authorizer {
    fn is_authorized(core_index: CoreIndex) -> AuthTrace {
        info!("Null Authorizer, [{core_index}], {} gas", gas());
        Default::default()
    }
}
