//! `jam build` command

use crate::{
    builder,
    manifest::{ModuleType, Profile},
};
use clap::Parser;

/// CLI utility for building PVM code blobs, particularly services and authorizers.
#[derive(Parser, Debug)]
pub struct Build {
    /// Path of crate to build; defaults to current directory if not supplied.
    path: Option<std::path::PathBuf>,
    /// Output path; defaults to `<crate-name>.pvm` in the current directory if not supplied.
    #[arg(short, long)]
    output: Option<std::path::PathBuf>,
    /// Module type to build.
    #[arg(short, long, value_enum, default_value_t = ModuleType::Automatic)]
    module: ModuleType,
    /// Install rustc dependencies if missing.
    #[arg(long)]
    auto_install: bool,
    /// The build profile to use.
    #[arg(short, long, value_enum, default_value_t = Profile::Release)]
    profile: Profile,
}

impl Build {
    /// Run the build command.
    pub fn run(&self) -> Result<(), anyhow::Error> {
        let cd = std::env::current_dir().expect("Unable to get current directory");
        let crate_dir = self.path.clone().unwrap_or_else(|| cd.clone());
        let blob_type = match self.module {
            ModuleType::Automatic => {
                let filename = crate_dir
                    .file_name()
                    .and_then(|x| x.to_str())
                    .expect("Invalid path?");
                let con_serv = filename.contains("service");
                let con_auth = filename.contains("authorizer");
                let con_corevm = filename.contains("corevm");
                if filename.ends_with("-service") || (con_serv && !con_auth && !con_corevm) {
                    builder::BlobType::Service
                } else if filename.ends_with("-authorizer")
                    || (!con_serv && con_auth && !con_corevm)
                {
                    builder::BlobType::Authorizer
                } else if filename.ends_with("-guest") || (!con_serv && !con_auth && con_corevm) {
                    builder::BlobType::CoreVmGuest
                } else {
                    panic!("Could not determine module type from crate name");
                }
            }
            ModuleType::Service => builder::BlobType::Service,
            ModuleType::Authorizer => builder::BlobType::Authorizer,
            ModuleType::CoreVmGuest => builder::BlobType::CoreVmGuest,
        };

        let out_dir = etc::find_up("target")?;
        let (crate_name, pvm_path) = builder::build_pvm_blob(
            &crate_dir,
            blob_type,
            out_dir.as_path(),
            self.auto_install,
            self.profile.clone().into(),
        );
        let output_file = self
            .output
            .clone()
            .unwrap_or_else(|| cd.join(format!("{crate_name}.jam")));
        std::fs::copy(pvm_path, &output_file).expect("Unable to write to output file");

        println!(
            "Written JAM-PVM blob for {} to {}...",
            crate_name,
            output_file.display()
        );

        Ok(())
    }
}
