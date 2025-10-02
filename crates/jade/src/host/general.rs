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
    use crate::prelude::{Vec, vec};
    use anyhow::Result;
    use service::vm::Operand;

    /// Fetch a value from the storage
    pub fn operands() -> Result<Vec<Operand>> {
        let len = unsafe { import::fetch(core::ptr::null_mut(), 0, 0, 14, 0, 0) };
        let mut target = vec![0; len as usize];
        let _ = unsafe { import::fetch(target.as_mut_ptr(), 0, len as u64, 14, 0, 0) };
        codec::decode(target.as_slice()).map_err(Into::into)
    }
}

/// Storage operations
pub mod storage {
    use super::*;
    use anyhow::Result;

    /// Read a value from the storage
    pub fn read<R: serde::de::DeserializeOwned>(key: impl AsRef<[u8]>) -> Option<R> {
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

        if len == u64::MAX {
            return None;
        } else if len == 0 {
            return None;
        }

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
        codec::decode(value).ok()
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
