#![cfg_attr(any(target_arch = "riscv32", target_arch = "riscv64"), no_std)]

extern crate alloc;
use alloc::string::String;
use jam_pvm_common::{is_authorized::*, *};
use jam_types::*;

#[allow(dead_code)]
struct Authorizer;
jam_pvm_common::declare_authorizer!(Authorizer);

impl jam_pvm_common::Authorizer for Authorizer {
	fn is_authorized(param: AuthConfig, package: WorkPackage, core_index: CoreIndex) -> AuthTrace {
		info!(
			"Null Authorizer, [{core_index}], {} gas, {param} param, {} token",
			gas(),
			package.authorization
		);
		if package.authorization.0 != param.0 {
			panic!("Authorization failed")
		}
		let m = String::from_utf8_lossy(&package.authorization);
		alloc::format!("Auth=<{m}>").as_bytes().to_vec().into()
	}
}
