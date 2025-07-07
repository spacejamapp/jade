use jam_types::{
	AccumulateItem, CodeHash, Hash, RefineContext, ServiceId, Slot, TransferRecord, Vec,
	WorkOutput, WorkPackageHash, WorkPayload,
};

/// Declare that this crate is a JAM service characterized by `$service_impl` and create necessary
/// entry points.
///
/// - `$service_impl` must implement the [Service] trait.
#[macro_export]
macro_rules! declare_service {
	($service_impl: ty) => {
		#[polkavm_derive::polkavm_export]
		extern "C" fn refine_ext(ptr: u32, size: u32) -> (u64, u64) {
			let $crate::jam_types::RefineParams {
				id,
				payload,
				package_hash,
				context,
				auth_code_hash,
			} = $crate::mem::decode_buf(ptr, size);
			let result = <$service_impl as $crate::Service>::refine(
				id,
				payload,
				package_hash,
				context,
				auth_code_hash,
			);
			((&result).as_ptr() as u64, result.len() as u64)
		}
		#[polkavm_derive::polkavm_export]
		extern "C" fn accumulate_ext(ptr: u32, size: u32) -> (u64, u64) {
			let $crate::jam_types::AccumulateParams { slot, id, results } =
				$crate::mem::decode_buf(ptr, size);
			let maybe_hash = <$service_impl as $crate::Service>::accumulate(slot, id, results);
			if let Some(hash) = maybe_hash {
				((&hash).as_ptr() as u64, 32u64)
			} else {
				(0, 0)
			}
		}
		#[polkavm_derive::polkavm_export]
		extern "C" fn on_transfer_ext(ptr: u32, size: u32) -> (u64, u64) {
			let $crate::jam_types::OnTransferParams { slot, id, transfers } =
				$crate::mem::decode_buf(ptr, size);
			<$service_impl as $crate::Service>::on_transfer(slot, id, transfers);
			(0, 0)
		}
	};
}

/// The invocation trait for a JAM service.
///
/// The [declare_service] macro requires that its parameter implement this trait.
pub trait Service {
	/// The Refine entry-point, used in-core on a single Work Item.
	///
	/// - `id`: The index of the service being refined.
	/// - `payload`: The payload data to process.
	/// - `package_hash`: The hash of the Work Package.
	/// - `context`: Various pieces of contextual information for the Refinement process.
	/// - `auth_code_hash`: The hash of the authorizer code which was used to authorize the Work
	///   Package.
	///
	/// Returns the Work Output, which will be passed into [Self::accumulate] in the on-chain
	/// (stateful) context.
	fn refine(
		id: ServiceId,
		payload: WorkPayload,
		package_hash: WorkPackageHash,
		context: RefineContext,
		auth_code_hash: CodeHash,
	) -> WorkOutput;

	/// The Accumulate entry-point, used on-chain on one or more Work Item Outputs, or possibly none
	/// in the case of an always-accumulate service.
	///
	/// - `slot`: The current time slot.
	/// - `id`: The service ID being accumulated.
	/// - `results`: The Work Outputs of the Work Items, together with additional information on the
	///   Work Packages which brought them about.
	fn accumulate(slot: Slot, id: ServiceId, results: Vec<AccumulateItem>) -> Option<Hash>;

	/// The On Transfer entry-point, used on-chain on one or more Transfers.
	///
	/// - `slot`: The current time slot.
	/// - `id`: The service ID being accumulated.
	/// - `transfers`: Information on the Transfers to the service ID in question.
	fn on_transfer(slot: Slot, id: ServiceId, transfers: Vec<TransferRecord>);
}
