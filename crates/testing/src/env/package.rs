//! Work package APIs

use crate::env::Env;
use anyhow::Result;

impl Env {
    /// Pack an execution item into the current work package
    pub fn send(&mut self, payload: Vec<u8>) -> Result<()> {
        Ok(())
    }
}
