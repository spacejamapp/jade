#[allow(unused_imports)]
use super::Authorizer;
use crate::{chain_params, opaque, FixedVec};
use bounded_collections::Get;
use codec::{Decode, Encode};
use core::sync::atomic::Ordering::Relaxed;

/// Version of the Gray Paper implemented by this and dependent crates.
pub const GP_VERSION: &str = "0.6.5";

/// Beginning of the Jam "Common Era" (1200 UTC on January 1, 2025),
/// as seconds after the Unix epoch.
pub const JAM_COMMON_ERA: u64 = 1_735_732_800;

/// Length of a transfer memo in bytes.
pub const MEMO_LEN: usize = 128;

/// Maximum length of the preimage in the encoded form.
#[doc(hidden)]
// TODO @ivan This is temporary value. Replace with the one from the GP when it's updated.
pub const MAX_PREIMAGE_LEN: usize = 4 * 1024 * 1024;

/// Maximum length of the preimage blob.
///
/// Equals `MAX_PREIMAGE_LEN` minus the overhead.
// TODO @ivan Unhide when the GP is updated.
#[doc(hidden)]
pub const MAX_PREIMAGE_BLOB_LEN: usize = MAX_PREIMAGE_LEN - 8;

/// PolkaVM page size in bytes.
pub const PAGE_SIZE: u32 = 4096;

#[cfg(not(feature = "tiny"))]
mod defaults {
	use super::ValIndex;
	pub(super) const VAL_COUNT: ValIndex = 1023;
	pub(super) const BASIC_PIECE_LEN: usize = 684;
}

#[cfg(feature = "tiny")]
mod defaults {
	use super::ValIndex;
	pub(super) const VAL_COUNT: ValIndex = 6;
	pub(super) const BASIC_PIECE_LEN: usize = 4;
}

chain_params! {
	/// Total number of validators in the JAM.
	static VAL_COUNT: _ = _(defaults::VAL_COUNT);
	pub fn val_count() -> ValIndex;
	pub struct ValCount; impl Get<_> for _ {}

	/// Number of bytes in a basic EC piece.
	static BASIC_PIECE_LEN: _ = _(defaults::BASIC_PIECE_LEN);
	pub fn basic_piece_len() -> usize;

	/// Number of authorizations in a queue allocated to a core.
	static AUTH_QUEUE_LEN: _ = _(80);
	pub fn auth_queue_len() -> usize;
	pub struct AuthQueueLen; impl Get<_> for _ {}

	/// Minimum period in blocks between going from becoming `Available` to `Zombie`, and then
	/// again from `Zombie` to non-existent.
	///
	/// This ensures firstly that any data added and referenced in a Work Report's lookup anchor
	/// block will remain on-chain right up until the latest possible time a dispute might
	/// conclude. Secondly, it ensure that we only need to record one "flip-flop" of the data's
	/// availability in order to be able to determine whether it's available or not at any block
	/// within this period.
	static MIN_TURNAROUND_PERIOD: _ = _(28_800);
	pub fn min_turnaround_period() -> Slot;

	/// Maximum number of Work Items in a Work Package.
	static MAX_WORK_ITEMS: _ = _(16);
	pub fn max_work_items() -> usize;
	pub struct MaxWorkItems; impl Get<_> for _ {}

	/// Maximum number of imports in a Work Package.
	static MAX_IMPORTS: _ = _(3072);
	pub fn max_imports() -> u32;
	pub struct MaxImports; impl Get<_> for _ {}

	/// Maximum number of exports in a Work Package.
	static MAX_EXPORTS: _ = _(3072);
	pub fn max_exports() -> u32;

	/// Maximum number of extrinsics in a Work Package.
	static MAX_EXTRINSICS: _ = _(128);
	pub fn max_extrinsics() -> u32;

	/// Maximum number of dependencies (total of prerequisites and SR lookup entries).
	static MAX_DEPENDENCIES: _ = _(8);
	pub fn max_dependencies() -> usize;

	/// Maximum size of a Work Package together with all extrinsic data and imported segments.
	static MAX_INPUT: _ = _(12 * 1024 * 1024);
	pub fn max_input() -> u32;

	/// Returns the number of bytes in a segment slice.
	pub fn segment_slice_len() -> usize {
		segment_len() / basic_piece_points()
	}
	pub struct SegmentSliceLen; impl Get<_> for _ {}

	/// Number of bytes in a segment. This is fixed.
	pub const SEGMENT_LEN: _ = 4104;
	pub fn segment_len() -> usize;
	pub struct SegmentLen; impl Get<_> for _ {}
}

/// Number of points in a piece.
pub fn basic_piece_points() -> usize {
	basic_piece_len() / POINT_LEN
}

/// Returns the number of pieces in a segment.
pub fn pieces_per_segment() -> usize {
	SEGMENT_LEN / basic_piece_len()
}

/// Baseline parameters for the JAM protocol.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Encode, Decode)]
pub struct Parameters {
	/// Total number of validators in the JAM. Must be divisible by guarantor group size (3).
	pub val_count: ValIndex,
	/// Number of octets in a basic piece. Must be even and divide into segment length (4,104).
	pub basic_piece_len: u32,
	/// Number of authorizations in a queue allocated to a core.
	pub auth_queue_len: u32,
	/// Minimum period in blocks between going from becoming `Available` to `Zombie`, and then
	/// again from `Zombie` to non-existent.
	pub min_turnaround_period: Slot,
	/// Maximum number of Work Items in a Work Package.
	pub max_work_items: u32,
	/// Maximum number of imports in a Work Package.
	pub max_imports: u32,
	/// Maximum number of exports in a Work Package.
	pub max_exports: u32,
	/// Maximum number of extrinsics in a Work Package.
	pub max_extrinsics: u32,
	/// Maximum number of dependencies (total of prerequisites and SR lookup entries).
	pub max_dependencies: u32,
	/// Maximum size of a Work Package together with all extrinsic data and imported segments.
	pub max_input: u32,
}

impl Parameters {
	pub fn get() -> Self {
		Self {
			val_count: val_count(),
			basic_piece_len: basic_piece_len() as u32,
			auth_queue_len: auth_queue_len() as u32,
			min_turnaround_period: min_turnaround_period(),
			max_work_items: max_work_items() as u32,
			max_imports: max_imports(),
			max_exports: max_exports(),
			max_extrinsics: max_extrinsics(),
			max_dependencies: max_dependencies() as u32,
			max_input: max_input(),
		}
	}
	pub fn validate(self) -> Result<(), &'static str> {
		if self.basic_piece_len % 2 != 0 {
			return Err("`basic_piece_len` is not even")
		}
		if SEGMENT_LEN % (self.basic_piece_len as usize) != 0 {
			return Err("`basic_piece_len` does not divide into `SEGMENT_LEN` (4,104)")
		}
		Ok(())
	}
	pub fn apply(self) -> Result<(), &'static str> {
		self.validate()?;
		VAL_COUNT.store(self.val_count, Relaxed);
		BASIC_PIECE_LEN.store(self.basic_piece_len as usize, Relaxed);
		AUTH_QUEUE_LEN.store(self.auth_queue_len as usize, Relaxed);
		MIN_TURNAROUND_PERIOD.store(self.min_turnaround_period, Relaxed);
		MAX_WORK_ITEMS.store(self.max_work_items as usize, Relaxed);
		MAX_IMPORTS.store(self.max_imports, Relaxed);
		MAX_EXPORTS.store(self.max_exports, Relaxed);
		MAX_EXTRINSICS.store(self.max_extrinsics, Relaxed);
		MAX_DEPENDENCIES.store(self.max_dependencies as usize, Relaxed);
		MAX_INPUT.store(self.max_input, Relaxed);
		Ok(())
	}
}

/// Number of bytes in an erasure-coding point.
pub const POINT_LEN: usize = 2;

/// Validators super-majority.
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct ValSuperMajority;
impl Get<u32> for ValSuperMajority {
	fn get() -> u32 {
		val_count() as u32 / 3 * 2 + 1
	}
}

/// Type that represents a time slot of six seconds.
///
/// This can be either in a relative sense or as a period which has elapsed from the Polkadot
/// Common Era, beginning 1200 UTC, 1 January 2025.
pub type Slot = u32;
/// Type to represent the index of a validator.
pub type ValIndex = u16;
/// Type to represent the index of a compute core.
pub type CoreIndex = u16;
/// Type to represent the index of a service.
pub type ServiceId = u32;
/// Type to represent a balance.
pub type Balance = u64;
/// Type which is double the length of Balance, for non-overflowing multiplies.
pub type DoubleBalance = u128;
/// Type to represent some gas which may be below zero. This is used primarily for the `invoke`
/// hostcall API which must be able to return a negative gas amount in case of a gas overrun.
pub type SignedGas = i64;
/// Type to represent some gas which must be at least zero.
pub type UnsignedGas = u64;
/// Type which is double the length of Gas, for non-overflowing multiplies.
pub type DoubleGas = u128;

/// A basic 256-bit data value.
///
/// This should generally not be used directly in the rich data types, but instead one of the
/// rich opaque hash types to avoid accidental misuse and provide pretty-print facilities.
pub type Hash = [u8; 32];

opaque! {
	/// Hash of an encoded block header.
	pub struct HeaderHash(pub [u8; 32]);

	/// Hash of PVM program code.
	pub struct CodeHash(pub [u8; 32]);

	/// Hash of an encoded Work Package.
	pub struct WorkPackageHash(pub [u8; 32]);

	/// Hash of an encoded Work Report.
	pub struct WorkReportHash(pub [u8; 32]);

	/// Hash of a Work Item's [WorkPayload].
	pub struct PayloadHash(pub [u8; 32]);

	/// Hash of the JAM state root.
	pub struct StateRootHash(pub [u8; 32]);

	/// Hash of an MMR peak.
	pub struct MmrPeakHash(pub [u8; 32]);

	/// Hash of an accumulation tree root node.
	pub struct AccumulateRootHash(pub [u8; 32]);

	/// Hash of a piece of extrinsic data.
	pub struct ExtrinsicHash(pub [u8; 32]);

	/// Hash of an encoded [Authorizer] value.
	pub struct AuthorizerHash(pub [u8; 32]);

	/// Hash of a segment tree root node.
	pub struct SegmentTreeRoot(pub [u8; 32]);

	/// Hash of a [Segment] value.
	pub struct SegmentHash(pub [u8; 32]);

	/// Hash of a Merkle tree node.
	pub struct MerkleNodeHash(pub [u8; 32]);

	/// Non usage-specific hash.
	///
	/// This can be useful for pretty-printing [type@Hash] values.
	pub struct AnyHash(pub [u8; 32]);

	/// Transfer memo data, included with balance transfers between services.
	pub struct Memo(pub [u8; MEMO_LEN]);

	/// Data constituting the Authorization Token in a Work Package.
	pub struct Authorization(pub Vec<u8>);

	/// PVM Program code.
	pub struct Code(pub Vec<u8>);

	/// Payload data defining a Work Item.
	pub struct WorkPayload(pub Vec<u8>);

	/// Authorization parameter.
	pub struct AuthConfig(pub Vec<u8>);

	/// Non usage-specific data.
	///
	/// This can be useful for pretty-printing `Vec<u8>` values.
	pub struct AnyVec(pub Vec<u8>);

	/// Output data of Refinement operation, passed into Accumulation.
	pub struct WorkOutput(pub Vec<u8>);

	/// Output data of Is Authorized operation, passed into both Refinement and Accumulation.
	pub struct AuthTrace(pub Vec<u8>);

	/// A Work Package Bundle, the aggregation of the Work Package, extrinsics, imports and import
	/// proofs.
	pub struct Bundle(pub Vec<u8>);

	/// Plain-old-data struct of the same length as an encoded Ed25519 public key.
	///
	/// This has no cryptographic functionality or dependencies.
	pub struct OpaqueEd25519Public(pub [u8; 32]);

	/// Plain-old-data struct of the same length as an encoded Bandersnatch public key.
	///
	/// This has no cryptographic functionality or dependencies.
	pub struct OpaqueBandersnatchPublic(pub [u8; 32]);

	/// Plain-old-data struct of the same length as an encoded BLS public key.
	///
	/// This has no cryptographic functionality or dependencies.
	pub struct OpaqueBlsPublic(pub [u8; 144]);

	/// Additional information on a validator, opaque to the actual usage.
	pub struct OpaqueValidatorMetadata(pub [u8; 128]);
}

/// A queue of [AuthorizerHash]s, each of which will be rotated into the authorizer pool for a core.
pub type AuthQueue = FixedVec<AuthorizerHash, AuthQueueLen>;

/// A segment of data.
pub type Segment = FixedVec<u8, SegmentLen>;
// TODO: ^^^ Measure performance penalty for this not being 4096.

pub trait ToAny {
	type Any;
	fn any(&self) -> Self::Any;
	fn into_any(self) -> Self::Any;
}

impl ToAny for [u8; 32] {
	type Any = AnyHash;
	fn any(&self) -> Self::Any {
		AnyHash(*self)
	}
	fn into_any(self) -> Self::Any {
		AnyHash(self)
	}
}

impl ToAny for alloc::vec::Vec<u8> {
	type Any = AnyVec;
	fn any(&self) -> Self::Any {
		AnyVec(self.clone())
	}
	fn into_any(self) -> Self::Any {
		AnyVec(self)
	}
}

impl ToAny for &[u8] {
	type Any = AnyVec;
	fn any(&self) -> Self::Any {
		AnyVec(self.to_vec())
	}
	fn into_any(self) -> Self::Any {
		AnyVec(self.to_vec())
	}
}
