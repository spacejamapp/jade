//! SpaceVM system interface

use crate::abi::Buffer;
use anyhow::Result;
use spacejam_service::{
    api::{AccumulateArgs, Accumulated, AuthorizeArgs, RefineArgs},
    service::result::{Executed, Refined},
};

/// Run the accumulate invocation
pub fn authorize(args: AuthorizeArgs) -> Result<Executed> {
    let encoded = serde_jam::encode(&args)?;
    let input = Buffer {
        ptr: encoded.as_ptr(),
        len: encoded.len(),
    };

    let output = unsafe { abi::authorize(input) };
    serde_jam::decode(output.as_slice()).map_err(Into::into)
}

/// Run the refine invocation
pub fn refine(args: RefineArgs) -> Result<Refined> {
    let encoded = serde_jam::encode(&args)?;
    let input = Buffer {
        ptr: encoded.as_ptr(),
        len: encoded.len(),
    };

    let output = unsafe { abi::refine(input) };
    serde_jam::decode(output.as_slice()).map_err(Into::into)
}

/// Run the accumulate invocation
pub fn accumulate(args: AccumulateArgs) -> Result<Accumulated> {
    let encoded = serde_jam::encode(&args)?;
    let input = Buffer {
        ptr: encoded.as_ptr(),
        len: encoded.len(),
    };

    let output = unsafe { abi::accumulate(input) };
    serde_jam::decode(output.as_slice()).map_err(Into::into)
}

mod abi {
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
        /// Run accumulate invocation
        pub fn accumulate(args: Buffer) -> Buffer;

        /// Run the refine invocation
        pub fn refine(args: Buffer) -> Buffer;

        /// Run the is_authorized invocation
        pub fn authorize(args: Buffer) -> Buffer;
    }
}
