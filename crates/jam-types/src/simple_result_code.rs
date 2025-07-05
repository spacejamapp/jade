/// Type which represents a result from a host call/"machine context mutator".
#[doc(hidden)]
pub type SimpleResult = u64;

#[repr(u64)]
#[doc(hidden)]
pub enum SimpleResultCode {
	/// `OK` General success.
	Ok = 0,
	/// `NONE` An item doesn't exist.
	Nothing = u64::MAX,
	/// `WHAT` Host call index invalid.
	HostCallInvalid = Self::Nothing as u64 - 1,
	/// `OOB` The buffer itself is invalid (cannot be accessed).
	OutOfBounds = Self::Nothing as u64 - 2,
	/// `WHO` Target service is unknown.
	IndexUnknown = Self::Nothing as u64 - 3,
	/// `FULL` Too much storage is used by the service for its holdings.
	StorageFull = Self::Nothing as u64 - 4,
	/// `CORE` Bad core index given.
	BadCore = Self::Nothing as u64 - 5,
	/// `CASH` The caller has too little funding.
	NoCash = Self::Nothing as u64 - 6,
	/// `LOW` The gas limit provided is too low (lower than the amount of gas required for the
	/// transfer).
	GasLimitTooLow = Self::Nothing as u64 - 7,
	/// `HUH` The item is already solicited or forgotten.
	ActionInvalid = Self::Nothing as u64 - 8,
}

impl From<SimpleResultCode> for SimpleResult {
	fn from(code: SimpleResultCode) -> Self {
		code as Self
	}
}

impl<E> From<SimpleResultCode> for Result<SimpleResult, E> {
	fn from(code: SimpleResultCode) -> Self {
		Ok(code as SimpleResult)
	}
}

#[doc(hidden)]
pub const LOWEST_ERROR: SimpleResult = SimpleResultCode::ActionInvalid as SimpleResult;

#[repr(u64)]
#[doc(hidden)]
pub enum InvokeOutcomeCode {
	/// `HALT` Completed normally.
	Halt = 0,
	/// `PANIC` Completed with a panic.
	Panic = 1,
	/// `FAULT` Completed with a page fault.
	PageFault = 2,
	/// `HOST` Completed with a host-call fault.
	HostCallFault = 3,
	/// `OOG` Completed by running out of gas.
	OutOfGas = 4,
}

impl From<InvokeOutcomeCode> for u64 {
	fn from(code: InvokeOutcomeCode) -> Self {
		code as Self
	}
}
