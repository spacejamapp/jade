use core::ops::{Add, AddAssign};

use crate::simple::{MaxImports, MaxWorkItems};

use super::*;
use simple::OpaqueBlsPublic;

/// Plain-old-data struct of the same length and layout to `ValKeyset` struct. This does not
/// bring in any cryptography.
#[derive(Copy, Clone, Encode, Decode, Debug, Eq, PartialEq)]
pub struct OpaqueValKeyset {
    /// The opaque Ed25519 public key.
    pub ed25519: OpaqueEd25519Public,
    /// The opaque Bandersnatch public key.
    pub bandersnatch: OpaqueBandersnatchPublic,
    /// The opaque BLS public key.
    pub bls: OpaqueBlsPublic,
    /// The opaque metadata.
    pub metadata: OpaqueValidatorMetadata,
}
impl Default for OpaqueValKeyset {
    fn default() -> Self {
        Self {
            ed25519: OpaqueEd25519Public::zero(),
            bandersnatch: OpaqueBandersnatchPublic::zero(),
            bls: OpaqueBlsPublic::zero(),
            metadata: OpaqueValidatorMetadata::zero(),
        }
    }
}

/// The opaque keys for each validator.
pub type OpaqueValKeysets = FixedVec<OpaqueValKeyset, ValCount>;

/// Reference to a sequence of import segments, which when combined with an index forms a
/// commitment to a specific segment of data.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RootIdentifier {
    /// Direct cryptographic commitment to the export-segments tree root.
    Direct(SegmentTreeRoot),
    /// Indirect reference to the export-segments tree root via a hash of the work-package which
    /// resulted in it.
    Indirect(WorkPackageHash),
}

impl From<SegmentTreeRoot> for RootIdentifier {
    fn from(root: SegmentTreeRoot) -> Self {
        Self::Direct(root)
    }
}
impl From<WorkPackageHash> for RootIdentifier {
    fn from(hash: WorkPackageHash) -> Self {
        Self::Indirect(hash)
    }
}
impl TryFrom<RootIdentifier> for SegmentTreeRoot {
    type Error = WorkPackageHash;
    fn try_from(root: RootIdentifier) -> Result<Self, Self::Error> {
        match root {
            RootIdentifier::Direct(root) => Ok(root),
            RootIdentifier::Indirect(hash) => Err(hash),
        }
    }
}

/// Import segments specification, which identifies a single exported segment.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ImportSpec {
    /// The identifier of a series of exported segments.
    pub root: RootIdentifier,
    /// The index into the identified series of exported segments.
    pub index: u16,
}

impl Encode for ImportSpec {
    fn encode_to<T: codec::Output + ?Sized>(&self, dest: &mut T) {
        let off = match &self.root {
            RootIdentifier::Direct(root) => {
                root.encode_to(dest);
                0
            }
            RootIdentifier::Indirect(hash) => {
                hash.encode_to(dest);
                1 << 15
            }
        };
        (self.index + off).encode_to(dest);
    }
}

impl Decode for ImportSpec {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        let h = Hash::decode(input)?;
        let i = u16::decode(input)?;
        let root = if i & (1 << 15) == 0 {
            SegmentTreeRoot::from(h).into()
        } else {
            WorkPackageHash::from(h).into()
        };
        Ok(Self {
            root,
            index: i & !(1 << 15),
        })
    }

    fn encoded_fixed_size() -> Option<usize> {
        Some(core::mem::size_of::<Hash>() + core::mem::size_of::<u16>())
    }
}

/// Specification of a single piece of extrinsic data.
#[derive(Clone, Encode, Decode, Debug)]
pub struct ExtrinsicSpec {
    /// The hash of the extrinsic data.
    pub hash: ExtrinsicHash,
    /// The length of the extrinsic data.
    pub len: u32,
}

/// Sequence of [WorkItem]s within a [WorkPackage] and thus limited in length to [max_work_items()].
pub type WorkItems = BoundedVec<WorkItem, MaxWorkItems>;

/// A definition of work to be done by the Refinement logic of a service and transformed into a
/// [WorkOutput] for its Accumulation logic.
#[derive(Clone, Encode, Decode, Debug)]
pub struct WorkItem {
    /// Service identifier to which this work item relates.
    pub service: ServiceId,
    /// The service's code hash at the time of reporting. This must be available in-core at the
    /// time of the lookup-anchor block.
    pub code_hash: CodeHash,
    /// Opaque data passed in to the service's Refinement logic to describe its workload.
    pub payload: WorkPayload,
    /// Gas limit with which to execute this work item's Refine logic.
    pub refine_gas_limit: UnsignedGas,
    /// Gas limit with which to execute this work item's Accumulate logic.
    pub accumulate_gas_limit: UnsignedGas,
    /// Sequence of imported data segments.
    pub import_segments: WorkItemImportsVec,
    /// Additional data available to the service's Refinement logic while doing its workload.
    pub extrinsics: Vec<ExtrinsicSpec>,
    /// Number of segments exported by this work item.
    pub export_count: u16,
}

/// A sequence of import specifications.
pub type WorkItemImportsVec = BoundedVec<ImportSpec, MaxImports>;

/// Various pieces of information helpful to contextualize the Refinement process.
#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq, Default)]
pub struct RefineContext {
    /// The most recent header hash of the chain when building. This must be no more than
    /// `RECENT_BLOCKS` blocks old when reported.
    pub anchor: HeaderHash,
    /// Must be state root of block `anchor`. This is checked on-chain when reported.
    pub state_root: StateRootHash,
    /// Must be Beefy root of block `anchor`. This is checked on-chain when reported.
    pub beefy_root: MmrPeakHash,
    /// The hash of a header of a block which is final. Availability will not succeed unless a
    /// super-majority of validators have attested to this.
    /// Preimage `lookup`s will be judged according to this block.
    ///
    /// NOTE: Storage pallet may not cycle more frequently than 48 hours (24 hours above
    ///   plus 24 hours dispute period).
    pub lookup_anchor: HeaderHash,
    /// The slot of `lookup_anchor` on the chain. This is checked in availability and the
    /// report's package will not be made available without it being correct.
    /// This value must be at least `anchor_slot + 14400`.
    pub lookup_anchor_slot: Slot,
    /// Hashes of Work Packages, the reports of which must be reported prior to this one.
    /// This is checked on-chain when reported.
    pub prerequisites: VecSet<WorkPackageHash>,
}

/// A work-package, a collection of work-items together with authorization and contextual
/// information. This is processed _in-core_ with Is-Authorized and Refine logic to produce a
/// work-report.
#[derive(Clone, Encode, Decode, Debug, Default)]
pub struct WorkPackage {
    /// Authorization token.
    pub authorization: Authorization,
    /// Service identifier.
    pub auth_code_host: ServiceId,
    /// Authorizer.
    pub authorizer: Authorizer,
    /// Refinement context.
    pub context: RefineContext,
    /// Sequence of work items.
    pub items: WorkItems,
}

/// The authorizer tuple which together identifies a means of determining whether a Work Package is
/// acceptable to execute on a core.
#[derive(Clone, Encode, Decode, Debug, Default)]
pub struct Authorizer {
    /// Authorization code hash.
    pub code_hash: CodeHash,
    /// Configuration blob for the auth logic.
    pub config: AuthConfig,
}

impl Authorizer {
    pub fn any() -> Self {
        Self {
            code_hash: CodeHash::zero(),
            config: Default::default(),
        }
    }

    pub fn with_concat<R>(&self, f: impl Fn(&[u8]) -> R) -> R {
        f(&[&self.code_hash.0[..], &self.config[..]].concat()[..])
    }
}

/// Potential errors encountered during the refinement of a [`WorkItem`].
///
/// Although additional errors may be generated internally by the PVM engine,
/// these are the specific errors designated by the GP for the [`WorkResult`]
/// and that are eligible to be forwarded to the accumulate process as part
/// of the [`AccumulateItem`].
#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq)]
#[doc(hidden)]
pub enum WorkError {
    /// Gas exhausted (∞).
    OutOfGas = 1,
    /// Unexpected termination (☇).
    Panic = 2,
    /// Invalid amount of segments exported.
    BadExports = 3,
    /// Bad code for the service (`BAD`).
    ///
    /// This may occur due to an unknown service identifier or unavailable code preimage.
    BadCode = 4,
    /// Out of bounds code size (`BIG`).
    CodeOversize = 5,
}

/// Fields describing the level of activity imposed on the core to construct the `WorkResult`
/// output.
#[derive(Copy, Clone, Encode, Decode, Debug, Eq, PartialEq, Default)]
#[doc(hidden)]
pub struct RefineLoad {
    /// The amount of gas actually used for this refinement.
    #[codec(compact)]
    pub gas_used: UnsignedGas,
    /// The number of imports made.
    #[codec(compact)]
    pub imports: u16,
    /// The number of extrinsics referenced.
    #[codec(compact)]
    pub extrinsic_count: u16,
    /// The amount of data used in extrinsics.
    #[codec(compact)]
    pub extrinsic_size: u32,
    /// The number of exports made.
    #[codec(compact)]
    pub exports: u16,
}

impl Add for RefineLoad {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            gas_used: self.gas_used + rhs.gas_used,
            imports: self.imports + rhs.imports,
            extrinsic_count: self.extrinsic_count + rhs.extrinsic_count,
            extrinsic_size: self.extrinsic_size + rhs.extrinsic_size,
            exports: self.exports + rhs.exports,
        }
    }
}

impl AddAssign for RefineLoad {
    fn add_assign(&mut self, rhs: Self) {
        self.gas_used += rhs.gas_used;
        self.imports += rhs.imports;
        self.extrinsic_count += rhs.extrinsic_count;
        self.extrinsic_size += rhs.extrinsic_size;
        self.exports += rhs.exports;
    }
}

/// The result and surrounding context of a single Refinement operation passed as part of a Work
/// Report.
#[derive(Clone, Encode, Decode, Debug, Eq, PartialEq)]
#[doc(hidden)]
pub struct WorkDigest {
    /// The service whose Refinement gave this result.
    pub service: ServiceId,
    /// The service's code hash at the time of reporting. This must be available in-core at the
    /// time of the lookup-anchor block.
    pub code_hash: CodeHash,
    /// The hash of the payload data passed into Refinement which gave this result.
    pub payload_hash: PayloadHash,
    /// The amount of gas to be used for the accumulation of this result.
    pub accumulate_gas: UnsignedGas,
    /// The result of the Refinement operation itself.
    #[codec(encoded_as = "CompactRefineResult")]
    pub result: Result<WorkOutput, WorkError>,
    /// Information the how much resources the refinement consumed.
    pub refine_load: RefineLoad,
}

/// The result and surrounding context of a single Refinement operation passed in to the
/// Accumulation logic.
#[derive(Debug, Encode, Decode, Clone)]
pub struct AccumulateItem {
    /// The hash of the work-package in which the work-item which gave this result was placed.
    pub package: WorkPackageHash,
    /// The root of the segment tree which was generated by the work-package in which the work-item
    /// which gave this result was placed.
    pub exports_root: SegmentTreeRoot,
    /// The hash of the authorizer which authorized the execution of the work-package
    /// which generated this result.
    pub authorizer_hash: AuthorizerHash,
    /// The hash of the payload data passed into Refinement which gave this result.
    pub payload: PayloadHash,
    /// The amount of gas provided to Accumulate by the work-item behind this result.
    #[codec(compact)]
    pub gas_limit: UnsignedGas,
    /// The result of the Refinement operation itself.
    #[codec(encoded_as = "CompactRefineResult")]
    pub result: Result<WorkOutput, WorkError>,
    /// The output of the Is-Authorized logic which authorized the execution of the work-package
    /// which generated this result.
    pub auth_output: AuthTrace,
}

/// Parameters for the invocation of Accumulate.
#[derive(Debug, Encode, Decode)]
#[doc(hidden)]
pub struct AccumulateParams {
    /// The current time slot.
    #[codec(compact)]
    pub slot: Slot,
    /// The index of the service being accumulated.
    #[codec(compact)]
    pub id: ServiceId,
    /// A sequence of work-results to accumulate.
    pub results: Vec<AccumulateItem>,
}

/// A single deferred transfer of balance and/or data, passed in to the invocation of On Transfer.
#[derive(Debug, Clone, Encode, Decode, Default)]
pub struct TransferRecord {
    /// The index of the service from which the transfer was made.
    pub source: ServiceId,
    /// The index of the service which is the target of the transfer.
    pub destination: ServiceId,
    /// The balance passed from the `source` service to the `destination`.
    pub amount: Balance,
    /// The information passed from the `source` service to the `destination`.
    pub memo: Memo,
    /// The gas limit with which the `destination` On Transfer logic may execute in order to
    /// process this transfer.
    pub gas_limit: UnsignedGas,
}

/// Parameters for the invocation of On Transfer.
#[derive(Debug, Encode, Decode)]
#[doc(hidden)]
pub struct OnTransferParams {
    /// The current time slot.
    #[codec(compact)]
    pub slot: Slot,
    /// The index of the service to which the transfers are being made.
    #[codec(compact)]
    pub id: ServiceId,
    /// The sequence of transfers to be processed.
    pub transfers: Vec<TransferRecord>,
}

// TODO: @gav Consider moving to jam-node.
/// Parameters for the invocation of On Transfer, reference variant.
#[derive(Debug, Encode)]
#[doc(hidden)]
pub struct OnTransferParamsRef<'a> {
    /// The current time slot.
    #[codec(compact)]
    pub slot: Slot,
    /// The index of the service to which the transfers are being made.
    #[codec(compact)]
    pub id: ServiceId,
    /// The sequence of transfers to be processed.
    pub transfers: &'a [TransferRecord],
}

/// Parameters for the invocation of Refine.
#[derive(Debug, Encode, Decode)]
#[doc(hidden)]
pub struct RefineParams {
    /// The index of the service being refined.
    #[codec(compact)]
    pub id: ServiceId,
    /// The payload data to process.
    pub payload: WorkPayload,
    /// The hash of the Work Package.
    pub package_hash: WorkPackageHash,
    /// Various pieces of contextual information for the Refinement process.
    pub context: RefineContext,
    /// The hash of the code of the authorizer which was used to authorize the Work Package.
    pub auth_code_hash: CodeHash,
}

// TODO: @gav Consider moving to jam-node.
/// Parameters for the invocation of Refine, reference variant.
#[derive(Debug, Encode)]
#[doc(hidden)]
pub struct RefineParamsRef<'a> {
    /// The index of the service being refined.
    #[codec(compact)]
    pub id: ServiceId,
    /// The payload data to process.
    pub payload: &'a WorkPayload,
    /// The hash of the Work Package.
    pub package_hash: &'a WorkPackageHash,
    /// Various pieces of contextual information for the Refinement process.
    pub context: &'a RefineContext,
    /// The hash of the code of the authorizer which was used to authorize the Work Package.
    pub auth_code_hash: &'a CodeHash,
}

/// Information concerning a particular service's state.
///
/// This is used in the `service_info` host-call.
#[derive(Debug, Clone, Encode, Decode, MaxEncodedLen)]
pub struct ServiceInfo {
    /// The hash of the code of the service.
    pub code_hash: CodeHash,
    /// The existing balance of the service.
    #[codec(compact)]
    pub balance: Balance,
    /// The minimum balance which the service must satisfy.
    #[codec(compact)]
    pub threshold: Balance,
    /// The minimum amount of gas which must be provided to this service's `accumulate` for each
    /// work item it must process.
    #[codec(compact)]
    pub min_item_gas: UnsignedGas,
    /// The minimum amount of gas which must be provided to this service's `on_transfer` for each
    /// memo (i.e. transfer receipt) it must process.
    #[codec(compact)]
    pub min_memo_gas: UnsignedGas,
    /// The total number of bytes used for data electively held for this service on-chain.
    #[codec(compact)]
    pub bytes: u64,
    /// The total number of items of data electively held for this service on-chain.
    #[codec(compact)]
    pub items: u32,
}

impl codec::ConstEncodedLen for ServiceInfo {}
/// Refine result used for compact encoding of work result as prescribed by GP.
struct CompactRefineResult(Result<WorkOutput, WorkError>);
struct CompactRefineResultRef<'a>(&'a Result<WorkOutput, WorkError>);

impl From<CompactRefineResult> for Result<WorkOutput, WorkError> {
    fn from(value: CompactRefineResult) -> Self {
        value.0
    }
}

impl<'a> From<&'a Result<WorkOutput, WorkError>> for CompactRefineResultRef<'a> {
    fn from(value: &'a Result<WorkOutput, WorkError>) -> Self {
        CompactRefineResultRef(value)
    }
}

impl<'a> codec::EncodeAsRef<'a, Result<WorkOutput, WorkError>> for CompactRefineResult {
    type RefType = CompactRefineResultRef<'a>;
}

impl Encode for CompactRefineResult {
    fn encode_to<T: codec::Output + ?Sized>(&self, dest: &mut T) {
        CompactRefineResultRef(&self.0).encode_to(dest)
    }
}

impl Encode for CompactRefineResultRef<'_> {
    fn encode_to<T: codec::Output + ?Sized>(&self, dest: &mut T) {
        match &self.0 {
            Ok(o) => {
                dest.push_byte(0);
                o.encode_to(dest)
            }
            Err(e) => e.encode_to(dest),
        }
    }
}

impl Decode for CompactRefineResult {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        match input.read_byte()? {
            0 => Ok(Self(Ok(WorkOutput::decode(input)?))),
            e => Ok(Self(Err(WorkError::decode(&mut &[e][..])?))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_refine_result_codec() {
        let enc_dec = |exp_res, exp_buf: &[u8]| {
            let buf = CompactRefineResultRef(&exp_res).encode();
            assert_eq!(buf, exp_buf);
            let res = CompactRefineResult::decode(&mut &buf[..]).unwrap();
            assert_eq!(res.0, exp_res);
        };

        enc_dec(Ok(vec![1, 2, 3].into()), &[0, 3, 1, 2, 3]);
        enc_dec(Err(WorkError::OutOfGas), &[1]);
        enc_dec(Err(WorkError::Panic), &[2]);
        enc_dec(Err(WorkError::BadExports), &[3]);
        enc_dec(Err(WorkError::BadCode), &[4]);
        enc_dec(Err(WorkError::CodeOversize), &[5]);
    }
}
