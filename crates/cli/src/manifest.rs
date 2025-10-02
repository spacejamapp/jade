//! Cargo manifest related utilities

use crate::builder::ProfileType;
use jam_program_blob::CrateInfo;
use std::{path::Path, process::Command};

/// Type of module to build
#[derive(clap::ValueEnum, Clone, Debug, Default)]
#[value(rename_all = "lowercase")]
pub enum ModuleType {
    /// Automatically derive the module type from the crate name.
    #[clap(alias = "auto")]
    #[default]
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

/// Type of profile to build
#[derive(clap::ValueEnum, Clone, Debug, Default)]
#[value(rename_all = "lowercase")]
pub enum Profile {
    /// The "debug" profile (debug symbols and no optimizations).
    Debug,
    /// The "release" profile (debug symbols and optimizations).
    #[default]
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

/// Get the crate info from the Cargo.toml file
pub fn crate_info(krate: &Path) -> anyhow::Result<CrateInfo> {
    let man = serde_json::from_str::<serde_json::Value>(&String::from_utf8(
        Command::new("cargo")
            .arg("read-manifest")
            .current_dir(krate)
            .output()?
            .stdout,
    )?)?;

    let name = man
        .get("name")
        .expect("could not find package name in Cargo.toml")
        .as_str()
        .expect("package name is not a string")
        .to_string();

    let version = man
        .get("version")
        .expect("could not find package version in Cargo.toml")
        .as_str()
        .expect("package version is not a string")
        .to_string();
    let license = man
        .get("license")
        .expect("could not find package license in Cargo.toml")
        .as_str()
        .expect("package license is not a string")
        .to_string();
    let authors = man
        .get("authors")
        .expect("could not find authors in Cargo.toml")
        .as_array()
        .expect("authors is not an array")
        .iter()
        .map(|x| x.as_str().expect("author is not a string").to_owned())
        .collect::<Vec<String>>();

    Ok(CrateInfo {
        name,
        version,
        license,
        authors,
    })
}
