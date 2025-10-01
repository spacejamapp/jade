#![cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), no_std)]

use jade::prelude::{AuthConfig, AuthTrace, CoreIndex, WorkPackage};

#[jade::is_authorized]
fn is_authorized(_param: AuthConfig, _package: WorkPackage, _core_index: CoreIndex) -> AuthTrace {
    Default::default()
}

/// The service blob for the null authorizer
#[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
pub const SERVICE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/service.jam"));
