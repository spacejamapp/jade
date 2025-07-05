use jam_types::{InvokeOutcomeCode, SimpleResult, SimpleResultCode, LOWEST_ERROR};

/// Error type for host-calls.
#[derive(Debug)]
pub enum ApiError {
	/// `OOB` Invalid memory access.
	OutOfBounds,
	/// `WHO` Target service is unknown.
	IndexUnknown,
	/// `FULL` Too much storage is used by the service for its holdings.
	StorageFull,
	/// `CORE` Bad core index given.
	BadCore,
	/// `CASH` The caller has too little funding.
	NoCash,
	/// `LOW` The gas limit provided is too low (lower than the amount of gas required for the
	/// transfer).
	GasLimitTooLow,
	/// `HUH` The hash is already solicited or forgotten.
	ActionInvalid,
}

impl From<u64> for ApiError {
	fn from(code: SimpleResult) -> Self {
		match code {
			c if c == SimpleResultCode::OutOfBounds as u64 => ApiError::OutOfBounds,
			c if c == SimpleResultCode::IndexUnknown as u64 => ApiError::IndexUnknown,
			c if c == SimpleResultCode::StorageFull as u64 => ApiError::StorageFull,
			c if c == SimpleResultCode::BadCore as u64 => ApiError::BadCore,
			c if c == SimpleResultCode::NoCash as u64 => ApiError::NoCash,
			c if c == SimpleResultCode::GasLimitTooLow as u64 => ApiError::GasLimitTooLow,
			c if c == SimpleResultCode::ActionInvalid as u64 => ApiError::ActionInvalid,
			_ => panic!("unknown error code: {}", code),
		}
	}
}

/// Result type for host-calls, parameterized by the type of the successful result.
pub type ApiResult<T> = Result<T, ApiError>;

/// Simple conversion trait for types which can be converted to a regular option.
pub trait IntoApiOption<T> {
	fn into_api_option(self) -> Option<T>;
}

impl IntoApiOption<()> for SimpleResult {
	fn into_api_option(self) -> Option<()> {
		if self == SimpleResultCode::Ok as u64 {
			Some(())
		} else if self == SimpleResultCode::Nothing as u64 {
			None
		} else {
			panic!("Our own API impl has resulted in success value which is out of range.");
		}
	}
}
impl IntoApiOption<u64> for SimpleResult {
	fn into_api_option(self) -> Option<u64> {
		if self < LOWEST_ERROR {
			Some(self)
		} else if self == SimpleResultCode::Nothing as u64 {
			None
		} else {
			panic!("Our own API impl has resulted in success value which is out of range.");
		}
	}
}
impl IntoApiOption<u32> for SimpleResult {
	fn into_api_option(self) -> Option<u32> {
		if self <= u32::MAX as _ {
			Some(self as _)
		} else if self == SimpleResultCode::Nothing as u64 {
			None
		} else {
			panic!("Our own API impl has resulted in success value which is out of range.");
		}
	}
}
impl IntoApiOption<usize> for SimpleResult {
	fn into_api_option(self) -> Option<usize> {
		if self < LOWEST_ERROR && self <= usize::MAX as _ {
			Some(self as _)
		} else if self == SimpleResultCode::Nothing as u64 {
			None
		} else {
			panic!("Our own API impl has resulted in success value which is out of range.");
		}
	}
}

/// Simple conversion trait for types which can be converted to a regular host-call API result.
pub trait IntoApiResult<T> {
	fn into_api_result(self) -> ApiResult<T>;
}

impl IntoApiResult<()> for SimpleResult {
	fn into_api_result(self) -> ApiResult<()> {
		if self == SimpleResultCode::Ok as u64 {
			Ok(())
		} else {
			Err(self.into())
		}
	}
}
impl IntoApiResult<u64> for SimpleResult {
	fn into_api_result(self) -> ApiResult<u64> {
		if self < LOWEST_ERROR {
			Ok(self)
		} else {
			Err(self.into())
		}
	}
}
impl IntoApiResult<usize> for SimpleResult {
	fn into_api_result(self) -> ApiResult<usize> {
		if self <= usize::MAX as _ && self < LOWEST_ERROR {
			Ok(self as usize)
		} else if self < LOWEST_ERROR {
			panic!("Our own API impl has resulted in success value which is out of range.");
		} else {
			Err(self.into())
		}
	}
}
impl IntoApiResult<u32> for SimpleResult {
	fn into_api_result(self) -> ApiResult<u32> {
		if self <= u32::MAX as _ {
			Ok(self as u32)
		} else if self < LOWEST_ERROR {
			panic!("Our own API impl has resulted in success value which is out of range.");
		} else {
			Err(self.into())
		}
	}
}
impl IntoApiResult<Option<()>> for SimpleResult {
	fn into_api_result(self) -> ApiResult<Option<()>> {
		if self < LOWEST_ERROR {
			Ok(Some(()))
		} else if self == SimpleResultCode::Nothing as u64 {
			Ok(None)
		} else {
			Err(self.into())
		}
	}
}
impl IntoApiResult<Option<u64>> for SimpleResult {
	fn into_api_result(self) -> ApiResult<Option<u64>> {
		if self < LOWEST_ERROR {
			Ok(Some(self))
		} else if self == SimpleResultCode::Nothing as u64 {
			Ok(None)
		} else {
			Err(self.into())
		}
	}
}
impl IntoApiResult<Option<usize>> for SimpleResult {
	fn into_api_result(self) -> ApiResult<Option<usize>> {
		if self <= usize::MAX as _ && self < LOWEST_ERROR {
			Ok(Some(self as usize))
		} else if self < LOWEST_ERROR {
			panic!("Our own API impl has resulted in success value which is out of range.");
		} else if self == SimpleResultCode::Nothing as u64 {
			Ok(None)
		} else {
			Err(self.into())
		}
	}
}

/// The successful result of inner PVM invocations.
#[derive(Clone, Copy)]
pub enum InvokeOutcome {
	/// `HALT` Completed normally.
	Halt,
	/// `FAULT` Completed with a page fault.
	PageFault(u64),
	/// `HOST` Completed with a host-call fault.
	HostCallFault(u64),
	/// `PANIC` Completed with a panic.
	Panic,
	/// `OOG` Completed by running out of gas.
	OutOfGas,
}

/// The result of the [crate::refine::invoke] host-call.
pub type InvokeResult = ApiResult<InvokeOutcome>;

/// Simple trait to convert to `InvokeResult`.
pub trait IntoInvokeResult {
	/// Convert `self` to `InvokeResult`.
	fn into_invoke_result(self) -> InvokeResult;
}

impl IntoInvokeResult for (u64, u64) {
	fn into_invoke_result(self) -> InvokeResult {
		const STATUS_HALT: u64 = InvokeOutcomeCode::Halt as u64;
		const STATUS_PANIC: u64 = InvokeOutcomeCode::Panic as u64;
		const STATUS_FAULT: u64 = InvokeOutcomeCode::PageFault as u64;
		const STATUS_HOST: u64 = InvokeOutcomeCode::HostCallFault as u64;
		const STATUS_OOG: u64 = InvokeOutcomeCode::OutOfGas as u64;
		// Convert `invoke` return value to `Result`.
		match self {
			(STATUS_HALT, _) => Ok(InvokeOutcome::Halt),
			(STATUS_FAULT, address) => Ok(InvokeOutcome::PageFault(address)),
			(STATUS_HOST, index) => Ok(InvokeOutcome::HostCallFault(index)),
			(STATUS_PANIC, _) => Ok(InvokeOutcome::Panic),
			(STATUS_OOG, _) => Ok(InvokeOutcome::OutOfGas),
			(code, _) => Err(code.into()),
		}
	}
}
