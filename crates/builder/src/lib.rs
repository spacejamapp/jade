//! Spacejam builder library

use anyhow::Result;
use jam_types::WorkPackage;
use std::future::Future;

/// A builder for building JAM service extrinsics
pub trait Builder {
    /// Add an work item to the builder
    fn send(&mut self, service: u32, auth: Vec<u8>, payload: Vec<u8>) -> Result<()>;

    /// Yield extrinsic data at a given timeslot
    fn extrinsic(&self, service: u32, timeslot: u32) -> Result<Vec<u8>>;

    /// Pack work packages at a given timeslot
    fn pack(&self, timeslot: u32) -> Result<Vec<WorkPackage>>;

    /// Submit a work package
    fn submit(&self, _package: WorkPackage) -> impl Future<Output = Result<()>> {
        async move { Ok(()) }
    }
}
