use crate::{
	imports,
	result::{ApiResult, IntoApiOption as _, IntoApiResult as _, IntoInvokeResult as _},
	ApiError, InvokeOutcome,
};
use alloc::{vec, vec::Vec};
use codec::Encode;
use core::{
	mem::{size_of, size_of_val, MaybeUninit},
	ptr,
};
use jam_types::*;

/// Check whether a preimage is available for lookup in the service.
///
/// - `hash`: The hash of the preimage to check availability.
///
/// Returns `true` if the preimage is available, `false` otherwise.
///
/// NOTE: Internally this uses the `historical_lookup` host call.
pub fn is_historical_available(hash: &[u8; 32]) -> bool {
	raw_foreign_historical_lookup_into(u64::MAX, hash, &mut []).is_some()
}

/// Check whether a preimage is available for lookup in another service.
///
/// - `service_id`: The service in whose preimage store to check availability.
/// - `hash`: The hash of the preimage to check availability.
///
/// Returns `true` if the preimage is available, `false` otherwise.
///
/// NOTE: Internally this uses the `historical_lookup` host call.
pub fn is_foreign_historical_available(service_id: ServiceId, hash: &[u8; 32]) -> bool {
	raw_foreign_historical_lookup_into(service_id as _, hash, &mut []).is_some()
}

/// Make a lookup into the service's preimage store without allocating.
///
/// - `hash`: The hash of the preimage to look up.
/// - `output`: The buffer to write the preimage into.
///
/// Returns the number of bytes written into the output buffer or `None` if the preimage was not
/// available.
///
/// NOTE: Internally this uses the `historical_lookup` host call.
pub fn historical_lookup_into(hash: &[u8; 32], output: &mut [u8]) -> Option<usize> {
	raw_foreign_historical_lookup_into(u64::MAX, hash, output)
}

/// Make a lookup into another service's preimage store without allocating.
///
/// - `service_id`: The service in whose preimage store to find the preimage.
/// - `hash`: The hash of the preimage to look up.
/// - `output`: The buffer to write the preimage into.
///
/// Returns the number of bytes written into the output buffer or `None` if the preimage was not
/// available.
///
/// NOTE: Internally this uses the `historical_lookup` host call.
pub fn foreign_historical_lookup_into(
	service_id: ServiceId,
	hash: &[u8; 32],
	output: &mut [u8],
) -> Option<usize> {
	raw_foreign_historical_lookup_into(service_id as _, hash, output)
}

/// Make a lookup into the service's preimage store.
///
/// - `hash`: The hash of the preimage to look up.
///
/// Returns the preimage or `None` if the preimage was not available.
///
/// NOTE: Internally this uses the `historical_lookup` host call.
pub fn historical_lookup(hash: &[u8; 32]) -> Option<Vec<u8>> {
	raw_foreign_historical_lookup(u64::MAX, hash)
}

/// Make a lookup into another service's preimage store.
///
/// - `service_id`: The service in whose preimage store to find the preimage.
/// - `hash`: The hash of the preimage to look up.
///
/// Returns the preimage or `None` if the preimage was not available.
///
/// NOTE: Internally this uses the `historical_lookup` host call.
pub fn foreign_historical_lookup(service_id: ServiceId, hash: &[u8; 32]) -> Option<Vec<u8>> {
	raw_foreign_historical_lookup(service_id as _, hash)
}

/// A definition of data to be fetched.
#[derive(Copy, Clone, Debug)]
pub enum Fetch {
	/// The current work-package.
	WorkPackage,
	/// The configuration of the authorization code.
	AuthCodeConfig,
	/// The input provided to the parameterized authorizer code.
	AuthToken,
	/// The output from the parameterized authorizer code.
	AuthTrace,
	/// A given work-item's payload.
	AnyPayload(usize),
	/// A particular extrinsic of a given work-item.
	AnyExtrinsic {
		/// The index of the work-item whose extrinsic should be fetched.
		work_item: usize,
		/// The index of the work-item's extrinsic to be fetched.
		index: usize,
	},
	/// A particular extrinsic of the executing work-item.
	OurExtrinsic(usize),
	/// A particular import-segment of a given work-item.
	AnyImport {
		/// The index of the work-item whose import-segment should be fetched.
		work_item: usize,
		/// The index of the work-item's import-segment to be fetched.
		index: usize,
	},
	/// A particular import-segment of the executing work-item.
	OurImport(usize),
}

impl Fetch {
	fn args(self) -> (u64, u64, u64) {
		use Fetch::*;
		match self {
			WorkPackage => (0, 0, 0),
			AuthCodeConfig => (1, 0, 0),
			AuthToken => (2, 0, 0),
			AuthTrace => (3, 0, 0),
			AnyPayload(index) => (4, index as _, 0),
			AnyExtrinsic { work_item, index } => (5, work_item as _, index as _),
			OurExtrinsic(index) => (6, index as _, 0),
			AnyImport { work_item, index } => (7, work_item as _, index as _),
			OurImport(index) => (8, index as _, 0),
		}
	}

	/// Fetch the data defined by this [Fetch] into the given target buffer.
	///
	/// - `target`: The buffer to write the fetched data into.
	/// - `skip`: The number of bytes to skip from the start of the data to be fetched.
	///
	/// Returns the full length of the data which is being fetched. If this is smaller than the
	/// `target`'s length, then some of the buffer will not be written to. If the request does not
	/// identify any data to be fetched (e.g. because an index is out of range) then returns `None`.
	pub fn fetch_into(self, target: &mut [u8], skip: usize) -> Option<usize> {
		let (kind, a, b) = self.args();
		unsafe {
			imports::fetch(target.as_mut_ptr(), skip as _, target.len() as _, kind as _, a, b)
		}
		.into_api_option()
	}

	/// Fetch the length of the data defined by this [Fetch].
	///
	/// Returns the length of the data which is being fetched. If the request does not identify any
	/// data to be fetched (e.g. because an index is out of range) then returns `None`.
	#[allow(clippy::len_without_is_empty)]
	pub fn len(self) -> Option<usize> {
		self.fetch_into(&mut [], 0)
	}

	/// Fetch the data defined by this [Fetch] into a newly allocated [Vec].
	///
	/// Returns a [Vec] containing the data identified by the value of `self`. If the request does
	/// not identify any data to be fetched (e.g. because an index is out of range) then returns
	/// `None`.
	pub fn fetch(self) -> Option<Vec<u8>> {
		let len = self.len()?;
		let mut incoming = vec![0u8; len];
		self.fetch_into(&mut incoming, 0)?;
		Some(incoming)
	}
}

/// Import a segment of data specified in the Work Item's import manifest.
///
/// - `index`: The index of the segment within the Work Items's import manifest to import.
///
/// Returns `Some` segment or `None` depending on whether the index references an import or not.
pub fn import(index: usize) -> Option<Segment> {
	let mut incoming = Segment::default();
	Fetch::OurImport(index).fetch_into(incoming.as_mut(), 0).map(|_| incoming)
}

/// Import a segment of data specified in a given Work Item's import manifest.
///
/// - `work_item`: The index of the work item to fetch an imported segment of.
/// - `index`: The index of the segment within the Work Items's import manifest to import.
///
/// Returns `Some` segment or `None` depending on whether the indices reference an import or not.
pub fn any_import(work_item: usize, index: usize) -> Option<Segment> {
	let mut incoming = Segment::default();
	Fetch::AnyImport { work_item, index }
		.fetch_into(incoming.as_mut(), 0)
		.map(|_| incoming)
}

/// Export a segment of data into the JAM Data Lake.
///
/// - `segment`: The segment of data to export.
///
/// Returns the export index or `Err` if the export was unsuccessful.
pub fn export(segment: &Segment) -> ApiResult<u64> {
	unsafe { imports::export(segment.as_ref().as_ptr(), segment.len() as u64) }.into_api_result()
}

/// Export a slice of data into the JAM Data Lake.
///
/// - `segment`: The slice of data to export, which may be no longer than [jam_types::SEGMENT_LEN].
///
/// Returns the export index or `Err` if the export was unsuccessful.
pub fn export_slice(segment: &[u8]) -> ApiResult<u64> {
	unsafe { imports::export(segment.as_ptr(), segment.len() as u64) }.into_api_result()
}

/// Create a new instance of a PVM.
///
/// - `code`: The code of the PVM.
/// - `program_counter`: The initial program counter value of the PVM.
///
/// Returns the handle of the PVM or `Err` if the creation was unsuccessful.
pub fn machine(code: &[u8], program_counter: u64) -> ApiResult<u64> {
	unsafe { imports::machine(code.as_ptr(), code.len() as u64, program_counter) }.into_api_result()
}

/// Inspect the raw memory of an inner PVM.
///
/// - `vm_handle`: The handle of the PVM whose memory to inspect.
/// - `inner_src`: The address in the PVM's memory to start reading from.
/// - `len`: The number of bytes to read.
///
/// Returns the data in the PVM `vm_handle` at memory `inner_src` or `Err` if the inspection failed.
pub fn peek(vm_handle: u64, inner_src: u64, len: u64) -> ApiResult<Vec<u8>> {
	let mut incoming = vec![0; len as usize];
	unsafe { imports::peek(vm_handle, incoming.as_mut_ptr(), inner_src, len) }
		.into_api_result()
		.map(|()| incoming)
}

/// Inspect the raw memory of an inner PVM.
///
/// - `vm_handle`: The handle of the PVM whose memory to inspect.
/// - `outer_dst`: The buffer to write the memory into.
/// - `inner_src`: The address in the PVM's memory to start reading from.
///
/// Returns `Ok` on success or `Err` if the inspection failed.
pub fn peek_into(vm_handle: u64, outer_dst: &mut [u8], inner_src: u64) -> ApiResult<()> {
	unsafe {
		imports::peek(vm_handle, outer_dst.as_mut_ptr(), inner_src, size_of_val(outer_dst) as u64)
	}
	.into_api_result()
}

/// Inspect a plain-old-data value in the memory of an inner PVM.
///
/// - `vm_handle`: The handle of the PVM whose memory to inspect.
/// - `inner_src`: The address in the PVM's memory to inspect a value of type `T`.
///
/// Returns the value of type `T` at `inner_src` of the PVM `vm_handle` or `Err` if the inspection
/// failed.
///
/// NOTE: This will only work with types `T` which have exactly the same memory layout in the host
/// and the inner PVM. Avoid things like references.
pub fn peek_value<T>(vm_handle: u64, inner_src: u64) -> ApiResult<T> {
	let mut t = MaybeUninit::<T>::uninit();
	unsafe {
		imports::peek(vm_handle, t.as_mut_ptr() as *mut u8, inner_src, size_of::<T>() as u64)
			.into_api_result()
			.map(|()| t.assume_init())
	}
}

/// Copy some data into the memory of an inner PVM.
///
/// - `vm_handle`: The handle of the PVM whose memory to mutate.
/// - `outer_src`: The data to be copied.
/// - `inner_dst`: The address in memory of inner PVM `vm_handle` to copy the data to.
///
/// Returns `Ok` on success or `Err` if the inspection failed.
pub fn poke(vm_handle: u64, outer_src: &[u8], inner_dst: u64) -> ApiResult<()> {
	unsafe { imports::poke(vm_handle, outer_src.as_ptr(), inner_dst, outer_src.len() as u64) }
		.into_api_result()
}

/// Copy a plain-old-data value into the memory of an inner PVM.
///
/// - `vm_handle`: The handle of the PVM whose memory to mutate.
/// - `outer_src`: The value whose memory representation is to be copied.
/// - `inner_dst`: The address in memory of inner PVM `vm_handle` to copy the value to.
///
/// Returns `Ok` on success or `Err` if the inspection failed.
pub fn poke_value<T>(vm_handle: u64, outer_src: &T, inner_dst: u64) -> ApiResult<()> {
	unsafe {
		imports::poke(
			vm_handle,
			outer_src as *const T as *const u8,
			inner_dst,
			size_of_val(outer_src) as u64,
		)
	}
	.into_api_result()
}

/// Initialize memory pages in an inner PVM with zeros, allocating if needed.
///
/// - `vm_handle`: The handle of the PVM whose memory to mutate.
/// - `page`: The index of the first page of inner PVM `vm_handle` to initialize.
/// - `count`: The number of pages to initialize.
///
/// Returns `Ok` on success or `Err` if the operation failed.
///
/// Pages are initialized to be filled with zeroes. If the pages are not yet allocated, they will
/// be allocated.
pub fn zero(vm_handle: u64, page: u64, count: u64) -> ApiResult<()> {
	unsafe { imports::zero(vm_handle, page, count) }.into_api_result()
}

/// Deallocate memory pages in an inner PVM.
///
/// - `vm_handle`: The handle of the PVM whose memory to mutate.
/// - `page`: The index of the first page of inner PVM `vm_handle` to deallocate.
/// - `count`: The number of pages to deallocate.
///
/// Returns `Ok` on success or `Err` if the operation failed.
///
/// NOTE: All pages from `page` to `page + count - 1` inclusive must have been allocated for this
/// call to succeed.
pub fn void(vm_handle: u64, page: u64, count: u64) -> ApiResult<()> {
	unsafe { imports::void(vm_handle, page, count) }.into_api_result()
}

/// Invoke an inner PVM.
///
/// - `vm_handle`: The handle of the PVM to invoke.
/// - `gas`: The maximum amount of gas which the inner PVM may use in this invocation.
/// - `regs`: The initial register values of the inner PVM.
///
/// Returns the outcome of the invocation, together with any remaining gas, and the final register
/// values.
pub fn invoke(
	vm_handle: u64,
	gas: SignedGas,
	regs: [u64; 13],
) -> ApiResult<(InvokeOutcome, SignedGas, [u64; 13])> {
	let mut args = InvokeArgs { gas, regs };
	let outcome = unsafe { imports::invoke(vm_handle, core::ptr::from_mut(&mut args).cast()) }
		.into_invoke_result()?;
	Ok((outcome, args.gas, args.regs))
}

/// Delete an inner PVM instance, freeing any associated resources.
///
/// - `vm_handle`: The handle of the PVM to delete.
///
/// Returns the inner PVM's final instruction counter value on success or `Err` if the operation
/// failed.
pub fn expunge(vm_handle: u64) -> ApiResult<u64> {
	unsafe { imports::expunge(vm_handle) }.into_api_result()
}

/// Inspect the gas meter.
///
/// Returns the post-instruction gas meter value.
pub fn gas() -> UnsignedGas {
	unsafe { imports::gas() }
}

/// Check whether a preimage is available for lookup.
///
/// - `hash`: The hash of the preimage to check availability.
///
/// Returns `true` if the preimage is available, `false` otherwise.
///
/// NOTE: Internally this uses the `lookup` host call.
pub fn is_available(hash: &[u8; 32]) -> bool {
	raw_foreign_lookup_into(u64::MAX, hash, &mut []).is_some()
}

/// Check whether a preimage is available for foreign lookup.
///
/// - `service_id`: The service in whose preimage store to check availability.
/// - `hash`: The hash of the preimage to check availability.
///
/// Returns `true` if the preimage is available, `false` otherwise.
///
/// NOTE: Internally this uses the `lookup` host call.
pub fn is_foreign_available(service_id: ServiceId, hash: &[u8; 32]) -> bool {
	raw_foreign_lookup_into(service_id as _, hash, &mut []).is_some()
}

/// Make a lookup into the service's preimage store without allocating.
///
/// - `hash`: The hash of the preimage to look up.
/// - `output`: The buffer to write the preimage into.
///
/// Returns the number of bytes written into the output buffer or `None` if the preimage was not
/// available.
///
/// NOTE: Internally this uses the `lookup` host call.
pub fn lookup_into(hash: &[u8; 32], output: &mut [u8]) -> Option<usize> {
	raw_foreign_lookup_into(u64::MAX, hash, output)
}

/// Make a lookup into another service's preimage store without allocating.
///
/// - `service_id`: The service in whose preimage store to find the preimage.
/// - `hash`: The hash of the preimage to look up.
/// - `output`: The buffer to write the preimage into.
///
/// Returns the number of bytes written into the output buffer or `None` if the preimage was not
/// available.
///
/// NOTE: Internally this uses the `lookup` host call.
pub fn foreign_lookup_into(
	service_id: ServiceId,
	hash: &[u8; 32],
	output: &mut [u8],
) -> Option<usize> {
	raw_foreign_lookup_into(service_id as _, hash, output)
}

/// Make a lookup into the service's preimage store.
///
/// - `hash`: The hash of the preimage to look up.
///
/// Returns the preimage or `None` if the preimage was not available.
///
/// NOTE: Internally this uses the `lookup` host call.
pub fn lookup(hash: &[u8; 32]) -> Option<Vec<u8>> {
	raw_foreign_lookup(u64::MAX, hash)
}

/// Make a lookup into another service's preimage store.
///
/// - `service_id`: The service in whose preimage store to find the preimage.
/// - `hash`: The hash of the preimage to look up.
///
/// Returns the preimage or `None` if the preimage was not available.
///
/// NOTE: Internally this uses the `lookup` host call.
pub fn foreign_lookup(service_id: ServiceId, hash: &[u8; 32]) -> Option<Vec<u8>> {
	raw_foreign_lookup(service_id as _, hash)
}

/// The status of a lookup request.
#[derive(Debug)]
pub enum LookupRequestStatus {
	/// The request has never had its preimage provided; corresponds to an empty GP array.
	Unprovided,
	/// The requested preimage is provided; corresponds to a single-item GP array.
	Provided {
		/// The slot at which the preimage was provided.
		since: Slot,
	},
	/// The request was provided and has since been unrequested; corresponds to a two-item GP
	/// array.
	Unrequested {
		/// The slot at which the preimage was provided.
		provided_since: Slot,
		/// The slot at which the preimage was unrequested.
		unrequested_since: Slot,
	},
	/// The request was provided, was since unrequested and is now requested again. Corresponds to
	/// a three-item GP array.
	Rerequested {
		/// The slot at which the preimage was provided.
		provided_since: Slot,
		/// The slot at which the preimage was unrequested.
		unrequested_at: Slot,
		/// The slot at which the preimage was requested again.
		rerequested_since: Slot,
	},
}

/// A summary of the implication of calling `forget` on a preimage request.
#[derive(Debug)]
pub enum ForgetImplication {
	/// The request will be dropped altogether since it was never provided. The deposit criteria
	/// will be lifted.
	Drop,
	/// The preimage will be unrequested and unavailable for lookup. No change in the deposit
	/// criteria.
	Unrequest,
	/// The preimage remain unavailable and be expunged from the state. The deposit criteria
	/// will be lifted.
	Expunge,
	/// The `forget` call is invalid and no change in state will be made.
	///
	/// If called in future, after `success_after`, it will be have the effect of `Unrequest`.
	NotYetUnrequest {
		/// The earliest slot at which a call to `forget` can succeed.
		success_after: Slot,
	},
	/// The `forget` call is invalid and no change in state will be made.
	///
	/// If called in future, after `success_after`, it will be have the effect of `Expunge`.
	NotYetExpunge {
		/// The earliest slot at which a call to `forget` can succeed.
		success_after: Slot,
	},
}

impl LookupRequestStatus {
	/// Return the implication of calling `forget` on the current state of the preimage request
	/// given the current timeslot is `now`.
	pub fn forget_implication(&self, now: Slot) -> ForgetImplication {
		match self {
			Self::Unprovided => ForgetImplication::Drop,
			Self::Provided { .. } => ForgetImplication::Unrequest,
			Self::Unrequested { unrequested_since, .. }
				if now > unrequested_since + min_turnaround_period() =>
				ForgetImplication::Drop,
			Self::Unrequested { unrequested_since, .. } => ForgetImplication::NotYetExpunge {
				success_after: unrequested_since + min_turnaround_period(),
			},
			Self::Rerequested { unrequested_at, .. }
				if now > unrequested_at + min_turnaround_period() =>
				ForgetImplication::Unrequest,
			Self::Rerequested { unrequested_at, .. } => ForgetImplication::NotYetUnrequest {
				success_after: unrequested_at + min_turnaround_period(),
			},
		}
	}
}

/// Query the status of a preimage.
///
/// - `hash`: The hash of the preimage to be queried.
/// - `len`: The length of the preimage to be queried.
///
/// Returns `Some` if `hash`/`len` has an active solicitation outstanding or `None` if not.
pub fn query(hash: &[u8; 32], len: usize) -> Option<LookupRequestStatus> {
	let (r0, r1): (u64, u64) = unsafe { imports::query(hash.as_ptr(), len as u64) };
	let n = r0 as u32;
	let x = (r0 >> 32) as Slot;
	let y = r1 as Slot;
	Some(match n {
		0 => LookupRequestStatus::Unprovided,
		1 => LookupRequestStatus::Provided { since: x },
		2 => LookupRequestStatus::Unrequested { provided_since: x, unrequested_since: y },
		3 => LookupRequestStatus::Rerequested {
			provided_since: x,
			unrequested_at: y,
			rerequested_since: (r1 >> 32) as Slot,
		},
		_ => return None,
	})
}

/// Request that preimage data be available for lookup.
///
/// - `hash`: The hash of the preimage to be made available.
/// - `len`: The length of the preimage to be made available.
///
/// Returns `Ok` on success or `Err` if the request failed.
///
/// [is_available] may be used to determine availability; once available, the preimage may be
/// fetched with [lookup] or its variants.
///
/// A preimage may only be solicited once for any service and soliciting a preimage raises the
/// minimum balance required to be held by the service.
pub fn solicit(hash: &[u8; 32], len: usize) -> Result<(), ApiError> {
	unsafe { imports::solicit(hash.as_ptr(), len as u64) }.into_api_result()
}

/// No longer request that preimage data be available for lookup, or drop preimage data once time
/// limit has passed.
///
/// - `hash`: The hash of the preimage to be forgotten.
/// - `len`: The length of the preimage to be forgotten.
///
/// Returns `Ok` on success or `Err` if the request failed.
///
/// This function is used twice in the lifetime of a requested preimage; once to indicate that the
/// preimage is no longer needed and again to "clean up" the preimage once the required duration
/// has passed. Whether it does one or the other is determined by the current state of the preimage
/// request.
pub fn forget(hash: &[u8; 32], len: usize) -> Result<(), ApiError> {
	unsafe { imports::forget(hash.as_ptr(), len as u64) }.into_api_result()
}

/// Set the default result hash of Accumulation.
///
/// - `hash`: The hash to be used as the Accumulation result.
///
/// This value will be returned from Accumulation on success. It may be overridden by further
/// calls to this function or by explicitly returning `Some` value from the
/// [crate::Service::accumulate] function. The [checkpoint] function may be used after a call to
/// this function to ensure that this value is returned in the case of an irregular termination.
pub fn yield_hash(hash: &[u8; 32]) {
	unsafe { imports::yield_hash(hash.as_ptr()) }
		.into_api_result()
		.expect("Cannot fail except for memory access; we provide a good address; qed")
}

/// Provide a requested preimage to any service.
pub fn provide(service_id: ServiceId, preimage: &[u8]) -> Result<(), ApiError> {
	unsafe { imports::provide(service_id as u64, preimage.as_ptr(), preimage.len() as _) }
		.into_api_result()
}

/// Fetch raw data from the service's key/value store.
///
/// - `key`: The key of the data to fetch.
///
/// Returns the data associated with the key or `None` if the key is not present.
pub fn get_storage(key: &[u8]) -> Option<Vec<u8>> {
	raw_get_foreign_storage(u64::MAX, key)
}

/// Fetch raw data from the service's key/value store into a buffer.
///
/// - `key`: The key of the data to fetch.
/// - `value`: The buffer to write the data into; on success, this is overwritten with the value
///   associated with `key` in the service's store, leaving any portions unchanged if the buffer is
///   longer than the value.
///
/// Returns the size of the data associated with the key or `None` if the key is not present.
pub fn get_storage_into(key: &[u8], value: &mut [u8]) -> Option<usize> {
	raw_get_foreign_storage_into(u64::MAX, key, value)
}

/// Fetch raw data from another service's key/value store.
///
/// - `id`: The ID of the service whose key/value store to fetch from.
/// - `key`: The key of the data to fetch.
///
/// Returns the data associated with the key in the key/value store of service `id` or `None` if
/// the key is not present.
pub fn get_foreign_storage(id: ServiceId, key: &[u8]) -> Option<Vec<u8>> {
	raw_get_foreign_storage(id as u64, key)
}

/// Fetch raw data from another service's key/value store into a buffer.
///
/// - `id`: The ID of the service whose key/value store to fetch from.
/// - `key`: The key of the data to fetch.
/// - `value`: The buffer to write the data into; on success, this is overwritten with the value
///   associated with `key` in said service's store, leaving any portions unchanged if the buffer is
///   longer than the value.
///
/// Returns the size of the data associated with the key in the key/value store of service `id` or
/// `None` if the key is not present.
pub fn get_foreign_storage_into(id: ServiceId, key: &[u8], value: &mut [u8]) -> Option<usize> {
	raw_get_foreign_storage_into(id as u64, key, value)
}

/// Fetch typed data from the service's key/value store.
///
/// - `key`: A value, whose encoded representation is the the key of the data to fetch.
///
/// Returns the decoded data associated with the key or `None` if the key is not present or the data
/// cannot be decoded into the type `R`.
pub fn get<R: Decode>(key: impl Encode) -> Option<R> {
	Decode::decode(&mut &key.using_encoded(get_storage)?[..]).ok()
}

/// Fetch typed data from another service's key/value store.
///
/// - `id`: The ID of the service whose key/value store to fetch from.
/// - `key`: A value, whose encoded representation is the the key of the data to fetch.
///
/// Returns the decoded data associated with the key in the key/value store of service `id` or
/// `None` if the key is not present or the data cannot be decoded into the type `R`.
pub fn get_foreign<R: Decode>(id: ServiceId, key: impl Encode) -> Option<R> {
	Decode::decode(&mut &key.using_encoded(|k| get_foreign_storage(id, k))?[..]).ok()
}

/// Set the value of a key to raw data in the service's key/value store.
///
/// - `key`: The key to be set.
/// - `data`: The data to be associated with the key.
///
/// Returns the previous value's length, which can be `None` if no value was associated
/// with the given `key`, or `Err` if the operation failed.
///
/// NOTE: If this key was not previously set or if the data is larger than the previous value, then
/// the minimum balance which the service must hold is raised and if the service has too little
/// balance then the call with fail with [ApiError::StorageFull].
pub fn set_storage(key: &[u8], data: &[u8]) -> Result<Option<usize>, ApiError> {
	unsafe { imports::write(key.as_ptr(), key.len() as u64, data.as_ptr(), data.len() as u64) }
		.into_api_result()
}

/// Remove a pair from the service's key/value store.
///
/// - `key`: The key to be removed.
///
/// Returns `Some` on success with the previous value's length, or `None` if the key does not exist.
///
/// NOTE: If the key does not exist, then the operation is a no-op.
pub fn remove_storage(key: &[u8]) -> Option<usize> {
	unsafe { imports::write(key.as_ptr(), key.len() as u64, ptr::null(), 0) }
		.into_api_result()
		.expect("Cannot fail except for memory access; we provide a good address; qed")
}

/// Set the value of a typed key to typed data in the service's key/value store.
///
/// - `key`: The value of an encodable type whose encoding is the key to be set.
/// - `value`: The value of an encodable type whose encoding be associated with said key.
///
/// Returns `Ok` on success or `Err` if the operation failed.
///
/// NOTE: If this key was not previously set or if the data is larger than the previous value, then
/// the minimum balance which the service must hold is raised and if the service has too little
/// balance then the call with fail with [ApiError::StorageFull].
pub fn set(key: impl Encode, value: impl Encode) -> Result<(), ApiError> {
	value.using_encoded(|v| key.using_encoded(|k| set_storage(k, v).map(|_| ())))
}

/// Remove a typed key from the service's key/value store.
///
/// - `key`: The value of an encodable type whose encoding is the key to be removed.
///
/// NOTE: If the key does not exist, then the operation is a no-op.
pub fn remove(key: impl Encode) {
	let _ = key.using_encoded(remove_storage);
}

/// Get information on the service.
///
/// Returns the value of [ServiceInfo] which describes the current state of the service.
pub fn my_info() -> ServiceInfo {
	raw_service_info(u64::MAX).expect("Current service must exist; qed")
}

/// Get information on another service.
///
/// - `id`: The ID of the service to get information on.
///
/// Returns the value of [ServiceInfo] which describes the current state of service `id`.
pub fn service_info(id: ServiceId) -> Option<ServiceInfo> {
	raw_service_info(id as _)
}

/// Create a new service.
///
/// - `code_hash`: The hash of the code of the service to create. The preimage of this hash will be
///   solicited by the new service and its according minimum balance will be transferred from the
///   executing service to the new service in order to fund it.
/// - `code_len`: The length of the code of the service to create.
/// - `min_item_gas`: The minimum gas required to be set aside for the accumulation of a single Work
///   Item in the new service.
/// - `min_memo_gas`: The minimum gas required to be set aside for any single transfer of funds and
///   corresponding processing of a memo in the new service.
///
/// Returns the new service ID or `Err` if the operation failed.
///
/// NOTE: This operation requires a balance transfer from the executing service to the new service
/// in order to succeed; if this would reduce the balance to below the minimum balance required,
/// then it will fail.
///
/// NOTE: This commits to the code of the new service but does not yet instantiate it; the code
/// preimage must be provided before the first Work Items of the new service can be processed.
pub fn create_service(
	code_hash: &CodeHash,
	code_len: usize,
	min_item_gas: UnsignedGas,
	min_memo_gas: UnsignedGas,
) -> Result<ServiceId, ApiError> {
	unsafe {
		imports::new(code_hash.as_ptr(), code_len as u64, min_item_gas, min_memo_gas)
			.into_api_result()
	}
}

/// Upgrade the code of the service.
///
/// - `code_hash`: The hash of the code to upgrade to, to be found in the service's preimage store.
/// - `min_item_gas`: The minimum gas required to be set aside for the accumulation of a single Work
///   Item in the new service.
/// - `min_memo_gas`: The minimum gas required to be set aside for any single transfer of funds and
///   corresponding processing of a memo in the new service.
///
/// NOTE: This commits to the new code of the service but does not yet instantiate it; the new code
/// preimage must be provided before the first Work Items of the new service can be processed.
/// Generally you should use [solicit] and [is_available] to ensure that the new code is already
/// in the service's preimage store and call this only as the final step in the process.
pub fn upgrade(code_hash: &CodeHash, min_item_gas: UnsignedGas, min_memo_gas: UnsignedGas) {
	unsafe { imports::upgrade(code_hash.as_ptr(), min_item_gas, min_memo_gas) }
		.into_api_result()
		.expect("Failure only in case of bad memory; it is good; qed")
}

/// "Upgrade" the service into an unexecutable zombie.
///
/// - `ejector`: The index of the service which will be able to call [eject] on the caller service
///   in order to finally delete it.
///
/// NOTE: This only sets the new code hash of the service but does not clear storage/preimages nor
/// [forget] the current code hash. Do these first!
pub fn zombify(ejector: ServiceId) {
	(ejector, [0; 28]).using_encoded(|data| {
		unsafe { imports::upgrade(data.as_ptr(), 0, 0) }
			.into_api_result()
			.expect("Failure only in case of bad memory; it is good; qed")
	})
}

/// Transfer data and/or funds to another service asynchronously.
///
/// - `destination`: The ID of the service to transfer to. This service must exist at present.
/// - `amount`: The amount of funds to transfer to the `destination` service. Reducing the services
///   balance by this amount must not result in it falling below the minimum balance required.
/// - `gas_limit`: The amount of gas to set aside for the processing of the transfer by the
///   `destination` service. This must be at least the service's [ServiceInfo::min_memo_gas]. The
///   effective gas cost of this call is increased by this amount.
/// - `memo`: A piece of data to give the `destination` service.
///
/// Returns `Ok` on success or `Err` if the operation failed.
///
/// NOTE: All transfers are deferred; they are guaranteed to be received by the destination service
/// in same time slot, but will not be processed synchronously with this call.
pub fn transfer(
	destination: ServiceId,
	amount: Balance,
	gas_limit: UnsignedGas,
	memo: &Memo,
) -> Result<(), ApiError> {
	unsafe {
		imports::transfer(destination as _, amount, gas_limit, memo.as_ref().as_ptr())
			.into_api_result()
	}
}

/// Remove the `target` zombie service, drop its final preimage item `code_hash` and transfer
/// remaining balance to this service.
///
/// - `target`: The ID of a zombie service which nominated the caller service as its ejector.
/// - `code_hash`: The hash of the only preimage item of the `target` service. It must be
///   unrequested and droppable.
///
/// Target must therefore satisfy several requirements:
/// - it should have a code hash which is simply the LE32-encoding of the caller service's ID;
/// - it should have only one preimage lookup item, `code_hash`;
/// - it should have nothing in its storage.
///
/// Returns `Ok` on success or `Err` if the operation failed.
pub fn eject(target: ServiceId, code_hash: &CodeHash) -> Result<(), ApiError> {
	unsafe { imports::eject(target as _, code_hash.as_ref().as_ptr()) }.into_api_result()
}

/// Reset the privileged services.
///
/// - `manager`: The ID of the service which may effectually call [bless] in the future.
/// - `assigner`: The ID of the service which may effectually call [assign] in the future.
/// - `designator`: The ID of the service which may effectually call [designate] in the future.
/// - `always_acc`: The list of service IDs which accumulate at least once in every JAM block,
///   together with the baseline gas they get for accumulation. This may be supplemented with
///   additional gas should there be Work Items for the service.
///
/// Returns `Ok` on success or `Err` if the operation failed.
///
/// NOTE: This service must be (or have been) the _manager_ service of JAM at the beginning of
/// accumulate for this call to have any effect. If the service is/was not _manager_, then it is
/// effectively a no-op.
pub fn bless<'a>(
	manager: ServiceId,
	assigner: ServiceId,
	designator: ServiceId,
	always_acc: impl IntoIterator<Item = &'a (ServiceId, UnsignedGas)>,
) {
	let data: Vec<u8> = always_acc.into_iter().flat_map(|x| x.encode()).collect();
	let len = data.len() as u64;
	unsafe { imports::bless(manager as _, assigner as _, designator as _, data.as_ptr(), len) }
		.into_api_result()
		.expect("Failure only in case of bad memory or bad service ID; both are good; qed")
}

/// Assign a series of authorizers to a core.
///
/// - `core`: The index of the core to assign the authorizers to.
/// - `auth_queue`: The authorizer-queue to assign to the core. These are a series of
///   [AuthorizerHash] values, which determine what kinds of Work Packages are allowed to be
///   executed on the core.
///
/// Returns `Ok` on success or `Err` if the operation failed. Failure can only happen if the value
/// of `core` is out of range.
///
/// NOTE: This service must be (or have been) the _assigner_ service of JAM at the beginning of
/// accumulate for this call to have any effect. If the service is/was not _assigner_, then it is
/// effectively a no-op.
pub fn assign(core: CoreIndex, auth_queue: &AuthQueue) -> Result<(), ApiError> {
	auth_queue
		.using_encoded(|d| unsafe { imports::assign(core as u64, d.as_ptr()) })
		.into_api_result()
}

/// Designate the new validator keys.
///
/// - `keys`: The new validator keys.
///
/// NOTE: This service must be (or have been) the _designator_ service of JAM at the beginning of
/// accumulate for this call to have any effect. If the service is/was not _designator_, then it is
/// effectively a no-op.
pub fn designate(keys: &OpaqueValKeysets) {
	keys.using_encoded(|d| unsafe { imports::designate(d.as_ptr()) })
		.into_api_result()
		.expect("Failure only in case of bad memory; it is good; qed")
}

/// Checkpoint the state of the accumulation at present.
///
/// In the case that accumulation runs out of gas or otherwise terminates unexpectedly, all
/// changes extrinsic to the machine state, such as storage writes and transfers, will be rolled
/// back to the most recent call to [checkpoint], or the beginning of the accumulation if no
/// checkpoint has been made.
pub fn checkpoint() {
	unsafe { imports::checkpoint() }
}

fn raw_foreign_lookup(service_id: u64, hash: &[u8; 32]) -> Option<Vec<u8>> {
	let maybe_len: Option<u64> =
		unsafe { imports::lookup(service_id, hash.as_ptr(), ptr::null_mut(), 0, 0) }
			.into_api_result()
			.expect("Cannot fail except for memory access; we provide a good address; qed");
	let len = maybe_len?;
	let mut incoming = vec![0; len as usize];
	unsafe {
		imports::lookup(service_id, hash.as_ptr(), incoming.as_mut_ptr(), 0, len);
	}
	Some(incoming)
}

fn raw_foreign_lookup_into(service_id: u64, hash: &[u8; 32], output: &mut [u8]) -> Option<usize> {
	let maybe_len: Option<u64> = unsafe {
		imports::lookup(service_id, hash.as_ptr(), output.as_mut_ptr(), 0, output.len() as u64)
	}
	.into_api_result()
	.expect("Cannot fail except for memory access; we provide a good address; qed");
	Some(maybe_len? as usize)
}

fn raw_foreign_historical_lookup(service_id: u64, hash: &[u8; 32]) -> Option<Vec<u8>> {
	let maybe_len: Option<u64> =
		unsafe { imports::historical_lookup(service_id, hash.as_ptr(), ptr::null_mut(), 0, 0) }
			.into_api_result()
			.expect("Cannot fail except for memory access; we provide a good address; qed");
	let len = maybe_len?;
	let mut incoming = vec![0; len as usize];
	unsafe {
		imports::historical_lookup(service_id, hash.as_ptr(), incoming.as_mut_ptr(), 0, len);
	}
	Some(incoming)
}

fn raw_foreign_historical_lookup_into(
	service_id: u64,
	hash: &[u8; 32],
	output: &mut [u8],
) -> Option<usize> {
	let maybe_len: Option<u64> = unsafe {
		imports::historical_lookup(
			service_id,
			hash.as_ptr(),
			output.as_mut_ptr(),
			0,
			output.len() as u64,
		)
	}
	.into_api_result()
	.expect("Cannot fail except for memory access; we provide a good address; qed");
	Some(maybe_len? as usize)
}

fn raw_service_info(service: u64) -> Option<ServiceInfo> {
	let mut buffer = vec![0u8; ServiceInfo::max_encoded_len()];
	let maybe_ok: Option<()> = unsafe { imports::info(service as _, buffer.as_mut_ptr()) }
		.into_api_result()
		.expect("Cannot fail except for memory access; we provide a good address; qed");
	maybe_ok?;
	ServiceInfo::decode(&mut &buffer[..]).ok()
}

fn raw_get_foreign_storage(id: u64, key: &[u8]) -> Option<Vec<u8>> {
	let maybe_len: Option<u64> =
		unsafe { imports::read(id as _, key.as_ptr(), key.len() as u64, ptr::null_mut(), 0, 0) }
			.into_api_result()
			.expect("Cannot fail except for memory access; we provide a good address; qed");
	let len = maybe_len?;
	if len == 0 {
		Some(vec![])
	} else {
		let mut incoming = vec![0; len as usize];
		unsafe {
			imports::read(id as _, key.as_ptr(), key.len() as u64, incoming.as_mut_ptr(), 0, len);
		}
		Some(incoming)
	}
}

fn raw_get_foreign_storage_into(id: u64, key: &[u8], value: &mut [u8]) -> Option<usize> {
	let r: ApiResult<Option<u64>> = unsafe {
		imports::read(
			id as _,
			key.as_ptr(),
			key.len() as _,
			value.as_mut_ptr(),
			0,
			value.len() as _,
		)
	}
	.into_api_result();
	Some(r.expect("Only fail is memory access; address is good; qed")? as usize)
}
