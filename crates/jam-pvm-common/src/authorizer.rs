use jam_types::{AuthConfig, AuthTrace, CoreIndex, WorkPackage};

/// Declare that this crate is a JAM authorizer characterized by `$auth_impl` and create necessary
/// entry points.
///
/// - `$auth_impl` must implement the [Authorizer] trait.
#[macro_export]
macro_rules! declare_authorizer {
	($auth_impl: ty) => {
		#[polkavm_derive::polkavm_export]
		extern "C" fn is_authorized_ext(ptr: u32, size: u32) -> (u64, u64) {
			use $crate::jam_types::{AuthConfig, CoreIndex, WorkPackage};
			let (param, package, core_index): (AuthConfig, WorkPackage, CoreIndex) =
				$crate::mem::decode_buf(ptr, size);
			let result =
				<$auth_impl as $crate::Authorizer>::is_authorized(param, package, core_index);
			((&result).as_ptr() as u64, result.len() as u64)
		}
	};
}

/// The invocation trait for a JAM authorizer.
///
/// The [declare_authorizer] macro requires that its parameter implement this trait.
pub trait Authorizer {
	/// The single entry-point of this PVM module, this determines whether a given Work Package
	/// should be authorized to run on a given core.
	///
	/// - `param`: The authorizer-parameter which parameterizes this logic in some way. (This can
	///   also be found in the Work Package itself, but it provided here for convenience.)
	/// - `package`: The Work Package to be authorized. It is guaranteed that the
	///   `package.authorizer.code_hash` identifies this Authorizer logic. The Work Package includes
	///   the `authorization` field which is freely settable by the Work Package builder in order to
	///   authorize the package against this (parameterized) authorizer.
	/// - `core_index`: The index of the core on which the Work Package will be executed.
	///
	/// Returns the authorization output, an opaque blob which will be passed into both Refine and
	/// Accumulate for all Work Items in `package`. If `package` is not authorized, then this should
	/// panic instead.
	fn is_authorized(param: AuthConfig, package: WorkPackage, core_index: CoreIndex) -> AuthTrace;
}
