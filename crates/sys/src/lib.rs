//! SpaceVM system interface

use crate::abi::Buffer;
pub use abi::init_logger;
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

    let output: Buffer = unsafe { abi::authorize(input) };
    codec::decode(&output.to_vec()).map_err(Into::into)
}

/// Run the refine invocation
pub fn refine(args: RefineArgs) -> Result<Refined> {
    let encoded = codec::encode(&args)?;
    let input = Buffer {
        ptr: encoded.as_ptr(),
        len: encoded.len(),
    };

    let output = unsafe { abi::refine(input) };
    codec::decode(&output.to_vec()).map_err(Into::into)
}

/// Run the accumulate invocation
pub fn accumulate(args: AccumulateArgs) -> Result<Accumulated> {
    let encoded = codec::encode(&args)?;
    let input = Buffer {
        ptr: encoded.as_ptr(),
        len: encoded.len(),
    };

    let output = unsafe { abi::accumulate(input) };
    codec::decode(&output.to_vec()).map_err(Into::into)
}

mod abi {
    #[cfg(feature = "interp")]
    pub use {
        interp_accumulate as accumulate, interp_authorize as authorize, interp_refine as refine,
    };

    #[cfg(not(feature = "interp"))]
    pub use {comp_accumulate as accumulate, comp_authorize as authorize, comp_refine as refine};

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct Buffer {
        pub ptr: *const u8,
        pub len: usize,
    }

    impl Buffer {
        /// Get the buffer as a byte slice
        pub fn to_vec(&self) -> Vec<u8> {
            let result = unsafe { core::slice::from_raw_parts(self.ptr, self.len).to_vec() };
            unsafe {
                let layout = std::alloc::Layout::from_size_align(self.len, 1).unwrap();
                std::alloc::dealloc(self.ptr as *mut _, layout);
            }
            result
        }
    }

    unsafe extern "C" {
        /// Initialize the logger
        pub fn init_logger(ansi: bool, timer: bool);

        /// Run the authorize invocation
        #[cfg(not(feature = "interp"))]
        pub fn comp_authorize(args: Buffer) -> Buffer;

        /// Run the refine invocation
        #[cfg(not(feature = "interp"))]
        pub fn comp_refine(args: Buffer) -> Buffer;

        /// Run the accumulate invocation
        #[cfg(not(feature = "interp"))]
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
