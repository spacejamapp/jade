//! The main JAM PVM API for creating authorizers and services on JAM. This includes a trait-based
//! invocation entry-point API as well as host-calls functions and types for working with them.
//!
//! In order to create a PVM executable containing a JAM service or authorizer, you must implement
//! the [Service] or [Authorizer] in some type and then pass your type into the `declare_service!`
//! or `declare_authorizer!` macro, respectively. This will generate the necessary entry-points for
//! the PVM to call into your implementation.
//!
//! ## Example Service
//! ```rust
//! extern crate alloc;
//! use alloc::vec::Vec;
//! use jam_pvm_common::{declare_service, Service, accumulate::set_storage};
//! use jam_types::*;
//!
//! struct MyService;
//! declare_service!(MyService);
//!
//! impl Service for MyService {
//!     fn refine(
//!         _id: ServiceId,
//!         payload: WorkPayload,
//!         _package_hash: WorkPackageHash,
//!         _context: RefineContext,
//!         _auth_code_hash: CodeHash,
//!     ) -> WorkOutput {
//!         [&b"Hello "[..], payload.take().as_slice()].concat().into()
//!     }
//!     fn accumulate(_slot: Slot, _id: ServiceId, items: Vec<AccumulateItem>) -> Option<Hash> {
//!         for item in items.into_iter() {
//!             if let Ok(data) = item.result {
//!                 set_storage(item.package.as_slice(), &data).expect("not enough balance?!");
//!             }
//!         }
//!         None
//!     }
//!     fn on_transfer(_slot: Slot, _id: ServiceId, _items: Vec<TransferRecord>) {}
//! }
//! ```
//!
//! ## Host-calls
//! The host-calls available to a service or authorizer are split into four modules:
//! - [is_authorized] for authorizers, to be called from the [Authorizer::is_authorized] function.
//! - [refine] for services, to be called from the [Service::refine] function.
//! - [accumulate] for services, to be called from the [Service::accumulate] function.
//! - [on_transfer] for services, to be called from the [Service::on_transfer] function.
//!
//! Each module contains a set of functions that can be called from the respective entry-point
//! function. These functions are used to interact with the PVM and the blockchain state.
//!
//! ## Logging
//! Five logging macros are provided similar to those of the `log` crate, [debug], [info], [warn],
//! [error], and [trace]. These macros are used with the non-standard PolkaJam `log` host-call and
//! the `format` macro. The host environment is responsible for forwarding these logs to the
//! appropriate destination.
//!
//! ## Features
//! - `authorizer`: Enables the authorizer API.
//! - `service`: Enables the service API.
//! - `logging`: Enables the logging service; without the logging macros will evaluate any operands
//!   but otherwise have no effect.
#![no_std]
#![allow(clippy::unwrap_used)]

extern crate alloc;

#[doc(hidden)]
pub use jam_types;

#[cfg(any(feature = "authorizer", doc))]
mod authorizer;
#[cfg(any(feature = "authorizer", doc))]
pub use authorizer::Authorizer;

#[cfg(any(feature = "service", doc))]
mod service;
#[cfg(any(feature = "service", doc))]
pub use service::Service;

#[allow(dead_code)]
mod host_calls;

/// Host-call APIs available for the [Authorizer::is_authorized] entry-point.
#[cfg(any(feature = "authorizer", doc))]
pub mod is_authorized {
	pub use super::host_calls::gas;
}

/// Host-call APIs available for the [Service::refine] entry-point.
#[cfg(any(feature = "service", doc))]
pub mod refine {
	pub use super::host_calls::{
		export, export_slice, expunge, foreign_historical_lookup as foreign_lookup,
		foreign_historical_lookup_into as foreign_lookup_into, gas, historical_lookup as lookup,
		historical_lookup_into as lookup_into, import, invoke,
		is_foreign_historical_available as is_foreign_available,
		is_historical_available as is_available, machine, peek, peek_into, peek_value, poke,
		poke_value, void, zero, Fetch,
	};
}

/// Host-call APIs available for the [Service::accumulate] entry-point.
#[cfg(any(feature = "service", doc))]
pub mod accumulate {
	pub use super::host_calls::{
		assign, bless, checkpoint, create_service, designate, eject, foreign_lookup,
		foreign_lookup_into, forget, gas, get, get_foreign, get_foreign_storage, get_storage,
		is_available, is_foreign_available, lookup, lookup_into, my_info, provide, query, remove,
		remove_storage, service_info, set, set_storage, solicit, transfer, upgrade, yield_hash,
		zombify, ForgetImplication, LookupRequestStatus,
	};
}

/// Host-call APIs available for the [Service::on_transfer] entry-point.
#[cfg(any(feature = "service", doc))]
pub mod on_transfer {
	pub use super::host_calls::{
		foreign_lookup, foreign_lookup_into, forget, gas, get, get_foreign, get_foreign_storage,
		get_storage, is_available, is_foreign_available, lookup, lookup_into, my_info, remove,
		remove_storage, service_info, set, set_storage, solicit,
	};
}

pub(crate) mod imports;

#[doc(hidden)]
pub mod logging;

#[doc(hidden)]
pub mod mem;

mod result;
pub use result::{ApiError, ApiResult, InvokeOutcome, InvokeResult};
