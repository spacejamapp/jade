//! Cargo manifest related utilities

use jam_program_blob::CrateInfo;

use crate::builder::ProfileType;
use std::{fs, path::Path};

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

/// Get the crate info from the Cargo.toml file
pub fn crate_info(krate: &Path) -> anyhow::Result<CrateInfo> {
    let manifest = krate.join("Cargo.toml");
    let manifest = fs::read_to_string(manifest)?;
    let man = toml::from_str::<toml::Value>(&manifest)?;
    let pkg = man
        .get("package")
        .expect("could not find package in Cargo.toml");
    let name = pkg
        .get("name")
        .expect("could not find package name in Cargo.toml")
        .to_string();

    let version = pkg
        .get("version")
        .expect("could not find package version in Cargo.toml")
        .to_string();
    let license = pkg
        .get("license")
        .expect("could not find package license in Cargo.toml")
        .to_string();
    let authors = pkg
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
