//! Configuration for the spacejam with hook node

use crate::config::Config;
use spacejam::{storage::Parity, validator::LocalValidator, Builder};

pub struct JadexSpec<Hook: runtime::Hook>(std::marker::PhantomData<Hook>);

impl<Hook: runtime::Hook + Send + Sync + 'static> runtime::Config for JadexSpec<Hook> {
    type Validator = LocalValidator;
    type Storage = Parity;
    type Vm = ();
    type Hook = Hook;
}

/// Start the spacejam node with custom hook
pub async fn start<Hook: runtime::Hook + Send + Sync + 'static>(
    builder: Builder,
    hook: Hook,
) -> anyhow::Result<()> {
    // TODO choose diff net
    builder
        .build_with_hook::<JadexSpec<Hook>>(hook)
        .await?
        .start()
        .await
}
