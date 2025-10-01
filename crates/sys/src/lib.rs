//! SpaceVM system interface

use crate::abi::Buffer;
use anyhow::Result;
use service::{
    api::{AccumulateArgs, Accumulated, AuthorizeArgs, RefineArgs},
    service::result::{Executed, Refined},
};

/// Run the accumulate invocation
pub fn authorize(args: AuthorizeArgs) -> Result<Executed> {
    let encoded = codec::encode(&args)?;
    let input = Buffer {
        ptr: encoded.as_ptr(),
        len: encoded.len(),
    };

    let output = unsafe { abi::authorize(input) };
    codec::decode(output.as_slice()).map_err(Into::into)
}

/// Run the refine invocation
pub fn refine(args: RefineArgs) -> Result<Refined> {
    let encoded = codec::encode(&args)?;
    let input = Buffer {
        ptr: encoded.as_ptr(),
        len: encoded.len(),
    };

    let output = unsafe { abi::refine(input) };
    codec::decode(output.as_slice()).map_err(Into::into)
}

/// Run the accumulate invocation
pub fn accumulate(args: AccumulateArgs) -> Result<Accumulated> {
    let encoded = codec::encode(&args)?;
    let input = Buffer {
        ptr: encoded.as_ptr(),
        len: encoded.len(),
    };

    let output = unsafe { abi::accumulate(input) };
    codec::decode(output.as_slice()).map_err(Into::into)
}

mod abi {
    #[cfg(feature = "interp")]
    pub use {
        interp_accumulate as accumulate, interp_authorize as authorize, interp_refine as refine,
    };

    #[cfg(not(feature = "interp"))]
    pub use {comp_accumulate as accumulate, comp_authorize as authorize, comp_refine as refine};

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Buffer {
        pub ptr: *const u8,
        pub len: usize,
    }

    impl Buffer {
        /// Get the buffer as a byte slice
        pub fn as_slice(&self) -> &[u8] {
            unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
        }
    }

    unsafe extern "C" {
        /// Run the authorize invocation
        pub fn comp_authorize(args: Buffer) -> Buffer;

        /// Run the refine invocation
        pub fn comp_refine(args: Buffer) -> Buffer;

        /// Run the accumulate invocation
        pub fn comp_accumulate(args: Buffer) -> Buffer;

        /// Run the is_authorized invocation
        #[cfg(feature = "interp")]
        pub fn interp_authorize(args: Buffer) -> Buffer;

        /// Run the refine invocation
        #[cfg(feature = "interp")]
        pub fn interp_refine(args: Buffer) -> Buffer;

        /// Run accumulate invocation
        #[cfg(feature = "interp")]
        pub fn interp_accumulate(args: Buffer) -> Buffer;
    }
}
