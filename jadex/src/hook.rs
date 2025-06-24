//! Hooks for the Jadex runtime

use std::ops::Deref;

/// A hook that is used in the runtime
pub struct JadexHook<T: runtime::Hook>(T);

impl<T: runtime::Hook> Deref for JadexHook<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: runtime::Hook + Send + Sync + 'static> runtime::Hook for JadexHook<T> {}
