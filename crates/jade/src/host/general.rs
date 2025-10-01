//! General host calls

use crate::host::import;
use core::ptr;

/// Get the gas used
pub fn gas() -> u64 {
    unsafe { import::gas() }
}

/// Fetch operations
pub mod fetch {
    use super::*;
    use crate::prelude::Vec;
    use anyhow::Result;
    use service::vm::Operand;

    /// Fetch a value from the storage
    pub fn operands() -> Result<Vec<Operand>> {
        let mut target = Vec::new();
        let len = unsafe { import::fetch(target.as_mut_ptr(), 0, 0, 14, 0, 0) };
        let _ = unsafe { import::fetch(target.as_mut_ptr(), 0, len as u64, 14, 0, 0) };
        codec::decode(target.as_slice()).map_err(Into::into)
    }
}

/// Storage operations
pub mod storage {
    use super::*;
    use anyhow::Result;

    /// Read a value from the storage
    pub fn read<R: serde::de::DeserializeOwned>(key: impl AsRef<[u8]>) -> Result<R> {
        let len = unsafe {
            import::read(
                u64::MAX as _,
                key.as_ref().as_ptr(),
                key.as_ref().len() as u64,
                ptr::null_mut(),
                0,
                0,
            )
        };

        let ptr = unsafe {
            import::read(
                u64::MAX as _,
                key.as_ref().as_ptr(),
                key.as_ref().len() as u64,
                ptr::null_mut(),
                0,
                len,
            )
        };

        let value = unsafe { core::slice::from_raw_parts(ptr as _, len as usize) };
        codec::decode(value).map_err(Into::into)
    }

    /// Write a value to the storage
    pub fn write<W: serde::Serialize>(key: impl AsRef<[u8]>, value: &W) -> Result<()> {
        let value = codec::encode(value)?;
        unsafe {
            import::write(
                key.as_ref().as_ptr(),
                key.as_ref().len() as u64,
                value.as_ptr(),
                value.len() as u64,
            );
        };

        Ok(())
    }
}
