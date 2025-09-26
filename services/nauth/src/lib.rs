#![cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), no_std)]

use jade::prelude::{AuthConfig, AuthTrace, CoreIndex, WorkPackage};

// #[jade::is_authorized]
pub fn is_authorized(
    _param: AuthConfig,
    _package: WorkPackage,
    _core_index: CoreIndex,
) -> AuthTrace {
    unimplemented!()
}
