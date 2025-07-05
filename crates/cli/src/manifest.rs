//! Cargo manifest related utilities

use std::{path::Path, process::Command};

use jam_pvm_builder::ProfileType;

#[derive(clap::ValueEnum, Clone, Debug)]
#[value(rename_all = "lowercase")]
pub enum ModuleType {
    /// Automatically derive the module type from the crate name.
    #[clap(alias = "auto")]
    Automatic,
    /// Service module.
    #[clap(alias = "serv")]
    Service,
    /// Authorizer module.
    #[clap(alias = "auth")]
    Authorizer,
    /// CoreVM guest code.
    #[clap(alias = "guest")]
    CoreVmGuest,
}

#[derive(clap::ValueEnum, Clone, Debug)]
#[value(rename_all = "lowercase")]
pub enum Profile {
    /// The "debug" profile (debug symbols and no optimizations).
    Debug,
    /// The "release" profile (debug symbols and optimizations).
    Release,
    /// The "production" profile (optimizations and no debug symbols).
    Production,
}
impl From<Profile> for ProfileType {
    fn from(profile: Profile) -> ProfileType {
        match profile {
            Profile::Debug => ProfileType::Debug,
            Profile::Release => ProfileType::Release,
            Profile::Production => ProfileType::Other("production"),
        }
    }
}

/// Validate the input manifest
pub fn validate(krate: &Path) -> anyhow::Result<()> {
    let manifest = Command::new("cargo")
        .current_dir(krate)
        .arg("read-manifest")
        .output()?
        .stdout;

    let man = serde_json::from_slice::<serde_json::Value>(&manifest)?;
    if man.get("name").is_none() {
        anyhow::bail!("Crate name not found in Cargo.toml");
    }

    if man.get("version").is_none() {
        anyhow::bail!("Crate version not found in Cargo.toml");
    }

    if man.get("license").is_none() {
        anyhow::bail!("Crate license not found in Cargo.toml");
    }

    if man.get("authors").is_none() {
        anyhow::bail!("Crate authors not found in Cargo.toml");
    }

    Ok(())
}
