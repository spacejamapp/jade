//! Configuration for the spacejam with hook node

use crate::config::{Config, Node};
use anyhow::Result;
use network::Network;
use spacejam::{Development, RuntimeSpec, SpaceJam, chain, spec, storage::Parity, validator::LocalValidator};
use std::{fs, net::SocketAddr, path::PathBuf, sync::Arc};

pub struct JadexSpec<Hook: runtime::Hook>(std::marker::PhantomData<Hook>);

impl<Hook: runtime::Hook + Send + Sync + 'static> runtime::Config for JadexSpec<Hook> {
    type Validator = LocalValidator;
    type Storage = Parity;
    type Vm = ();
    type Hook = Hook;
}

/// Start the spacejam node with custom hook
pub async fn start<Hook: runtime::Hook + Send + Sync + 'static>(
    config: &Config,
    hook: Hook,
) -> Result<()> {
    // TODO choose diff net

    build::<Hook, JadexSpec<Hook>>(&config.node, hook)
        .await?
        .start()
        .await
}

async fn build<Hook: runtime::Hook + Send + Sync + 'static, C: RuntimeSpec<Hook = Hook>>(
    node: &Node,
    hook: Hook,
) -> Result<SpaceJam<C>> {
    let genesis = if let Some(genesis) = &node.chain {
        serde_json::from_slice(fs::read(genesis)?.as_slice())?
    } else {
        chain::Spec::dev()
    }
    .parse()?;

    // apply config from the spec file
    //
    // TODO: handle bootnode and peer id
    // node.network.genesis = genesis.genesis_header.hash()?;

    // prepare the runtime
    let data = {
        let data = PathBuf::from(&node.data_path).join(genesis.id.to_string());
        if !data.exists() {
            fs::create_dir_all(&data)?;
        }
        data
    };

    let runtime = C::runtime_with_hook(node.validator.as_deref(), data, genesis, hook).await?;
    if node.dev {
        return Ok(SpaceJam::Dev(spec::Dev {
            runtime,
            rpc: node.rpc,
        }));
    }

    let network = Network::new(node.network.clone(), Arc::new(runtime)).await?;
    if node.light {
        return Ok(SpaceJam::Light(spec::Light {
            network,
            rpc: node.rpc,
        }));
    }

    Ok(SpaceJam::Validating(spec::Validating(network)))
}
