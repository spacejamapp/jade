//! SpaceJam node service

use crate::Config;
use anyhow::Result;
use network::Network;
use runtime::{Runtime, Validator};
use score::{
    Block,
    block::{self, Header},
    service::ServiceAccount,
};
use spacejam::{RuntimeSpec, chain, storage::Parity, validator::LocalValidator};
use std::{collections::BTreeMap, fs, marker, sync::Arc, time::Duration};

/// Start the node service
pub async fn start<Hook: runtime::Hook + Send + Sync + 'static>(
    config: &Config,
    hook: Hook,
) -> Result<()> {
    let (runtime, networkcfg) = self::runtime(config, hook).await?;
    let network = Network::<JadexSpec<Hook>>::new(networkcfg, Arc::new(runtime)).await?;
    network.spawn().await;
    Ok(())
}

/// Start the development node service
pub async fn dev<Hook: runtime::Hook + Send + Sync + 'static>(
    config: &Config,
    hook: Hook,
) -> Result<()> {
    let (mut runtime, _) = self::runtime(config, hook).await?;
    runtime.validator = <JadexSpec<Hook> as runtime::Config>::Validator::dev();
    let author = runtime.author();

    tracing::info!("Starting development spacejam node");
    loop {
        let now = block::now() as u64;
        let duration = (score::SLOT_PERIOD as u64 - (now % score::SLOT_PERIOD as u64)) as u64;
        tokio::time::sleep(Duration::from_secs(duration)).await;

        let timeslot = block::timeslot();
        let block = author.author(timeslot).await?;
        tracing::trace!(
            "block#{}@0x{}",
            block.header.slot,
            hex::encode(&block.header.hash()?[..3])
        );

        author.finalize(block).await?;
    }
    Ok(())
}

/// Build the runtime from config
async fn runtime<Hook: runtime::Hook + Send + Sync + 'static>(
    config: &Config,
    hook: Hook,
) -> Result<(Runtime<JadexSpec<Hook>>, network::Config)> {
    let chain = config.node.data.join("chain");
    let genesis = if let Some(path) = config.node.spec.as_ref().map(|p| p.join("spec.json")) {
        serde_json::from_slice(fs::read(&path)?.as_slice())?
    } else {
        chain::Spec::dev()
    }
    .parse()?;

    // build the network config
    let networkcfg = network::Config {
        address: config.node.quic,
        bootnodes: genesis.bootnodes.clone(),
        genesis: genesis.genesis_header.hash()?,
    };

    let runtime = JadexSpec::<Hook>::runtime_with_hook(None, chain, genesis, hook).await?;
    Ok((runtime, networkcfg))
}

/// The Jadex runtime spec
pub struct JadexSpec<Hook: runtime::Hook>(marker::PhantomData<Hook>);

impl<Hook: runtime::Hook + Send + Sync + 'static> runtime::Config for JadexSpec<Hook> {
    type Storage = Parity;
    type Validator = LocalValidator;
    type Vm = ();
    type Hook = Hook;
}
