//! Builder logic for creating PVM code blobs for execution on the JAM PVM instances (service code
//! and authorizer code).

#![allow(clippy::unwrap_used)]

// If you update this, you should also update the toolchain installed by .github/workflows/rust.yml
const TOOLCHAIN: &str = "nightly-2024-11-01";

use codec::Encode;
use jam_program_blob::{ConventionalMetadata, CoreVmProgramBlob, ProgramBlob};
use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::OnceLock,
};

use crate::manifest;

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
            Self::Service => vec![
                b"refine_ext".into(),
                b"accumulate_ext".into(),
                b"on_transfer_ext".into(),
            ],
            Self::Authorizer => vec![b"is_authorized_ext".into()],
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

fn build_pvm_blob_in_build_script(crate_name: &str, crate_dir: &Path, blob_type: BlobType) {
    let out_dir: PathBuf = std::env::var("OUT_DIR").expect("No OUT_DIR").into();
    let crate_dir = if !crate_dir.exists() {
        // This provided path doesn't exist - this probably means we're building one crate at a
        // time. It should still be available, but in the dependencies folder.
        println!("Provided source path invalid. Presume building from crates.io");
        let cd = std::env::current_dir().unwrap();
        println!("Current path: {}", cd.display());

        let lock = cd.join("Cargo.lock");
        if !lock.exists() {
            panic!("Cargo.lock not found in current directory. Presume building from crates.io");
        }
        let lock = fs::read_to_string(lock)
            .expect("Failed to read Cargo.lock")
            .parse::<toml::Value>()
            .unwrap();
        let package = lock["package"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|x| x.as_table().map(|x| x.to_owned()))
            .find(|x| x.get("name").unwrap().as_str().unwrap() == crate_name)
            .expect("Dependency not found in Cargo.lock. Cannot continue.");
        let version = package.get("version").unwrap().as_str().unwrap();

        println!("Found dependency {crate_name} in manifest of version {version}");
        let mut source_path = cd.clone();
        source_path.pop();
        source_path.push(format!("{crate_name}-{version}"));
        if source_path.exists() {
            println!("Found source path: {}", source_path.display());
            source_path
        } else {
            println!(
                "Dependency source not found at {}. Packages found:",
                source_path.display()
            );
            for entry in std::fs::read_dir(cd.parent().unwrap()).unwrap() {
                let entry = entry.unwrap();
                if entry.file_type().unwrap().is_dir() {
                    println!("  - {}", entry.file_name().to_string_lossy());
                }
            }
            panic!("Cannot continue.");
        }
    } else {
        crate_dir.to_owned()
    };
    println!("cargo:rerun-if-env-changed=SKIP_PVM_BUILDS");
    if std::env::var_os("SKIP_PVM_BUILDS").is_some() {
        let output_file = blob_type.output_file(&out_dir, crate_name);
        fs::write(&output_file, []).expect("error creating dummy program blob");
        println!("cargo:rustc-env=PVM_BINARY={}", output_file.display());
    } else {
        println!("cargo:rerun-if-changed={}", crate_dir.to_str().unwrap());
        let (_crate_name, output_file) =
            build_pvm_blob(&crate_dir, blob_type, &out_dir, false, ProfileType::Release);
        println!("cargo:rustc-env=PVM_BINARY={}", output_file.display());
    }
}

/// Build the service crate in `crate_dir` for the RISCV target, convert to PVM code and finish
/// by creating a `<crate_name>.pvm` blob file.
///
/// This is intended to be called from a crate's `build.rs`. The generated blob may be included in
/// the crate by using the `pvm_binary` macro.
pub fn build_service(crate_name: &str, crate_dir: &Path) {
    build_pvm_blob_in_build_script(crate_name, crate_dir, BlobType::Service);
}

/// Build the authorizer crate in `crate_dir` for the RISCV target, convert to PVM code and finish
/// by creating a `<crate_name>.pvm` blob file.
///
/// This is intended to be called from a crate's `build.rs`. The generated blob may be included in
/// the crate by using the `pvm_binary` macro.
pub fn build_authorizer(crate_name: &str, crate_dir: &Path) {
    build_pvm_blob_in_build_script(crate_name, crate_dir, BlobType::Authorizer);
}

/// Build the CoreVM guest program crate in `crate_dir` for the RISCV target, convert to PVM code
/// and finish by creating a `<crate_name>.pvm` blob file.
///
/// If used in `build.rs`, then this may be included in the relevant crate by using the `pvm_binary`
/// macro.
pub fn build_corevm(crate_name: &str, crate_dir: &Path) {
    build_pvm_blob_in_build_script(crate_name, crate_dir, BlobType::CoreVmGuest);
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
        .args(["build", "-Z", "build-std=core,alloc"])
        .arg(profile.to_arg())
        .arg("--target")
        .arg(target_json_path)
        .arg("--features")
        .arg(if !matches!(blob_type, BlobType::CoreVmGuest) {
            "tiny"
        } else {
            ""
        });

    // Use job server to not oversubscribe CPU cores when compiling multiple PVM binaries in
    // parallel.
    if let Some(client) = get_job_server_client() {
        client.configure(&mut child);
    }

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

    let orig = fs::read(&input_path)
        .unwrap_or_else(|e| panic!("Failed to read {input_path:?} :{e:?}"));
    let linked = polkavm_linker::program_from_elf(config, orig.as_ref())
        .expect("Failed to link pvm program:");

    // Write out a full `.pvm` blob for debugging/inspection.
    let output_path_pvm = &out_dir.join(format!("{}.pvm", &info.name));
    fs::write(output_path_pvm, &linked).expect("Error writing resulting binary");
    let name = info.name.clone();
    let metadata = ConventionalMetadata::Info(info).encode().into();
    let output_file = blob_type.output_file(out_dir, &name);
    if !matches!(blob_type, BlobType::CoreVmGuest) {
        let parts = polkavm_linker::ProgramParts::from_bytes(linked.into())
            .expect("failed to deserialize linked PolkaVM program");
        let blob = ProgramBlob::from_pvm(&parts, metadata)
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

fn get_job_server_client() -> Option<&'static jobserver::Client> {
    static CLIENT: OnceLock<Option<jobserver::Client>> = OnceLock::new();
    CLIENT
        .get_or_init(|| unsafe { jobserver::Client::from_env() })
        .as_ref()
}

#[macro_export]
macro_rules! pvm_binary {
    ($name:literal) => {
        include_bytes!(env!("PVM_BINARY"))
    };
}
