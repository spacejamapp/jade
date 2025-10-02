//! Utility functions for the PVM CLI

use crate::ModuleType;
use anyhow::Result;
use std::{env, fs, path::PathBuf};

/// Build the PVM blob
///
/// NOTE: this is used for the build script of services
pub fn build(package: &str, module: Option<ModuleType>) -> Result<()> {
    println!("rerun-if-changed=build.rs");
    println!("rerun-if-changed=src");

    // avoid shadowing build on riscv target
    let target = env::var("TARGET")?;
    if target.contains("polkavm") {
        return Ok(());
    }

    // Build the service
    let target = etc::find_up("target")?;
    let jam = target.join("jam");
    let parget = jam.join(package);
    let mut build = crate::cmd::Build::default();
    if let Some(module) = module {
        build.module = module;
    }
    build.target = Some(parget.clone());
    build.run()?;

    // copy service to OUT_DIR
    let name = format!("{package}.jam");
    let binary = parget.join("jam").join(&name);
    let service = PathBuf::from(env::var("OUT_DIR")?).join("service.jam");
    println!("Copying service to OUT_DIR: {}", service.display());
    fs::copy(&binary, &service)?;
    fs::copy(&binary, jam.join(name))?;
    Ok(())
}
