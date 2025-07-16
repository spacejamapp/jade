use codec::{Decode, Encode, MaxEncodedLen};

/// Args for invoking an inner PVM.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Encode, Decode, MaxEncodedLen, Default)]
pub struct InvokeArgs {
	pub gas: i64,
	pub regs: [u64; 13],
}

impl codec::ConstEncodedLen for InvokeArgs {}
