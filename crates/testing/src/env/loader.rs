//! Environment loader

use crate::Env;
use anyhow::Result;
use cjam::builder;
use std::fs;

impl Env {
    /// Load the environment for the current working package
    pub fn load() -> Result<Env> {
        let target = etc::find_up("target")?;
        let root = target
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Failed to find the root of the current service"))?;
        let (_name, path) = builder::build_pvm_blob(
            &root,
            builder::BlobType::Service,
            &target,
            true,
            builder::ProfileType::Release,
        );

        let code = fs::read(path)?;
        let mut env = Env::default();
        env.id = env.add_account(257, code);
        Ok(env)
    }
}
