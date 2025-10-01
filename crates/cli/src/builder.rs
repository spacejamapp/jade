//! Builder logic for creating PVM code blobs for execution on the JAM PVM instances (service code
//! and authorizer code).

#![allow(clippy::unwrap_used)]

// If you update this, you should also update the toolchain installed by .github/workflows/rust.yml
const TOOLCHAIN: &str = "1.90.0";

use crate::manifest;
use jam_codec::Encode;
use jam_program_blob::{ConventionalMetadata, CoreVmProgramBlob, ProgramBlob};
use polkavm_linker::ProgramParts;
use std::{
    borrow::Cow,
    fmt::Display,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

pub enum BlobType {
    Service,
    Authorizer,
    CoreVmGuest,
}
impl Display for BlobType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Service => write!(f, "Service"),
            Self::Authorizer => write!(f, "Authorizer"),
            Self::CoreVmGuest => write!(f, "CoreVmGuest"),
        }
    }
}

impl BlobType {
    pub fn dispatch_table(&self) -> Vec<Vec<u8>> {
        match self {
            Self::Service => vec![b"jade_refine".into(), b"jade_accumulate".into()],
            Self::Authorizer => vec![b"jade_is_authorized".into()],
            Self::CoreVmGuest => vec![b"main".into()],
        }
    }

    fn output_file(&self, out_dir: &Path, crate_name: &str) -> PathBuf {
        let suffix = match self {
            Self::Service | Self::Authorizer => "jam",
            Self::CoreVmGuest => "corevm",
        };
        out_dir.join(format!("{crate_name}.{suffix}"))
    }
}

pub enum ProfileType {
    Debug,
    Release,
    Other(&'static str),
}
impl ProfileType {
    fn as_str(&self) -> &'static str {
        match self {
            ProfileType::Debug => "debug",
            ProfileType::Release => "release",
            ProfileType::Other(s) => s,
        }
    }
    fn to_arg(&self) -> String {
        match self {
            ProfileType::Debug => "--debug".into(),
            ProfileType::Release => "--release".into(),
            ProfileType::Other(s) => format!("--profile={s}"),
        }
    }
}

/// Build the PVM crate in `crate_dir` called `crate_name` for the RISCV target, convert to PVM
/// code and finish by creating a `.pvm` blob file of path `output_file`. `out_dir` is used to store
/// any intermediate build files.
pub fn build_pvm_blob(
    crate_dir: &Path,
    blob_type: BlobType,
    out_dir: &Path,
    install_rustc: bool,
    profile: ProfileType,
) -> (String, PathBuf) {
    let (target_name, target_json_path) = (
        "riscv64emac-unknown-none-polkavm",
        polkavm_linker::target_json_64_path().unwrap(),
    );

    println!("🪤 PVM module type: {blob_type}");
    println!("🎯 Target name: {target_name}");

    let rustup_installed = if Command::new("rustup").output().is_ok() {
        let output = Command::new("rustup")
            .args(["component", "list", "--toolchain", TOOLCHAIN, "--installed"])
            .output()
            .unwrap_or_else(|_| {
                panic!(
				"Failed to execute `rustup component list --toolchain {TOOLCHAIN} --installed`.\n\
		Please install `rustup` to continue.",
			)
            });

        if !output.status.success()
            || !output
                .stdout
                .split(|x| *x == b'\n')
                .any(|x| x[..] == b"rust-src"[..])
        {
            if install_rustc {
                println!("Installing rustc dependencies...");
                let mut child = Command::new("rustup")
                    .args(["toolchain", "install", TOOLCHAIN, "-c", "rust-src"])
                    .stdout(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit())
                    .spawn()
                    .unwrap_or_else(|_| {
                        panic!(
						"Failed to execute `rustup toolchain install {TOOLCHAIN} -c rust-src`.\n\
				Please install `rustup` to continue."
					)
                    });
                if !child
                    .wait()
                    .expect("Failed to execute rustup process")
                    .success()
                {
                    panic!("Failed to install `rust-src` component of {TOOLCHAIN}.");
                }
            } else {
                panic!("`rust-src` component of {TOOLCHAIN} is required to build the PVM binary.",);
            }
        }
        println!("ℹ️ `rustup` and toolchain installed. Continuing build process...");

        true
    } else {
        println!("ℹ️ `rustup` not installed, here be dragons. Continuing build process...");

        false
    };

    let info = manifest::crate_info(crate_dir).expect("failed to get crate info");
    println!("📦 Crate name: {}", info.name);
    println!("🏷️ Build profile: {}", profile.as_str());

    let mut child = Command::new("cargo");

    child
        .current_dir(crate_dir)
        .env_clear()
        .env("PATH", std::env::var("PATH").unwrap())
        .env("RUSTFLAGS", "-C panic=abort")
        .env("CARGO_TARGET_DIR", out_dir)
        // Support building on stable. (required for `-Zbuild-std`)
        .env("RUSTC_BOOTSTRAP", "1");

    if rustup_installed {
        child.arg(format!("+{TOOLCHAIN}"));
    }

    child
        .args(["rustc", "-Z", "build-std=core,alloc"])
        .arg(profile.to_arg())
        .arg("--target")
        .arg(target_json_path)
        .arg("--features")
        .arg(if !matches!(blob_type, BlobType::CoreVmGuest) {
            "tiny"
        } else {
            ""
        })
        .arg("--lib")
        .arg("--crate-type=cdylib")
        .env(
            "RUSTFLAGS",
            format!(
                "{} -C link-arg=--strip-debug",
                std::env::var("RUSTFLAGS").unwrap_or_default()
            ),
        );

    let mut child = child.spawn().expect("Failed to execute cargo process");
    let status = child.wait().expect("Failed to execute cargo process");

    if !status.success() {
        eprintln!("Failed to build RISC-V ELF due to cargo execution error");
        std::process::exit(1);
    }

    // Post processing
    println!("Converting RISC-V ELF to PVM blob...");
    let mut config = polkavm_linker::Config::default();
    config.set_strip(true);
    config.set_dispatch_table(blob_type.dispatch_table());

    let input_root = &out_dir.join(target_name).join(profile.as_str());
    let input_path_bin = input_root.join(&info.name);
    let input_path_cdylib = input_root.join(format!("{}.elf", info.name.replace("-", "_")));
    let input_path = if input_path_cdylib.exists() {
        if input_path_bin.exists() {
            eprintln!(
                "Both {} and {} exist; run 'cargo clean' to get rid of old artifacts!",
                input_path_cdylib.display(),
                input_path_bin.display()
            );
            std::process::exit(1);
        }
        input_path_cdylib
    } else if input_path_bin.exists() {
        input_path_bin
    } else {
        eprintln!(
            "Failed to build: neither {} nor {} exist",
            input_path_cdylib.display(),
            input_path_bin.display()
        );
        std::process::exit(1);
    };

    let orig =
        fs::read(&input_path).unwrap_or_else(|e| panic!("Failed to read {input_path:?} :{e:?}"));
    let linked = polkavm_linker::program_from_elf(config, orig.as_ref())
        .expect("Failed to link pvm program:");

    // Write out a full `.pvm` blob for debugging/inspection.
    fs::create_dir_all(out_dir).expect("Failed to create jam directory");
    let output_path_pvm = out_dir.join(format!("{}.pvm", &info.name));
    fs::write(output_path_pvm, &linked).expect("Error writing resulting binary");
    let name = info.name.clone();
    let metadata = ConventionalMetadata::Info(info).encode().into();
    let output_file = blob_type.output_file(out_dir, &name);
    if !matches!(blob_type, BlobType::CoreVmGuest) {
        let parts = polkavm_linker::ProgramParts::from_bytes(linked.into())
            .expect("failed to deserialize linked PolkaVM program");
        let blob = self::to_blob(&parts, metadata)
            .to_vec()
            .expect("error serializing the .jam blob");
        fs::write(&output_file, blob).expect("error writing the .jam blob");
    } else {
        let blob = CoreVmProgramBlob {
            metadata,
            pvm_blob: linked.into(),
        }
        .to_vec()
        .expect("error serializing the CoreVM blob");
        fs::write(&output_file, blob).expect("error writing the CoreVM blob");
    }

    (name, output_file)
}

#[macro_export]
macro_rules! pvm_binary {
    ($name:literal) => {
        include_bytes!(env!("PVM_BINARY"))
    };
}

fn to_blob<'a>(parts: &'a ProgramParts, metadata: Cow<'a, [u8]>) -> ProgramBlob<'a> {
    // Pad RO section with zeroes.
    let mut ro_data = parts.ro_data.to_vec();
    ro_data.resize(parts.ro_data_size as usize, 0);
    // Calculate the padding for RW section.
    let padding = (parts.rw_data_size as usize).next_multiple_of(4096)
        - parts.rw_data.len().next_multiple_of(4096);
    let rw_data_padding_pages = padding / 4096;
    let rw_data_padding_pages = rw_data_padding_pages
        .try_into()
        .expect("The RW data section is too big");
    ProgramBlob {
        metadata,
        ro_data: ro_data.into(),
        rw_data: (&parts.rw_data[..]).into(),
        code_blob: (&parts.code_and_jump_table[..]).into(),
        rw_data_padding_pages,
        stack_size: parts.stack_size,
    }
}
