//! Link spacevm to this sys library

use platforms::*;
use std::{
    env, fs,
    io::Result,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

const LIB_BASE: &str = "https://github.com/spacejamapp/specjam/releases/download/0.7.1-pre.1";
const LIB_NAME: &str = "spacevm-0.7.1";
const PLATFORMS: [&str; 4] = ["linux-amd64", "linux-arm64", "macos-amd64", "macos-arm64"];

fn main() -> std::io::Result<()> {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let libs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("lib");
    self::download_libs(&libs)?;

    // copy the lib to out dir
    let target = out.join(LIB);
    let src = libs.join(PLATFORM).join(LIB);
    fs::copy(src, target)?;

    // link the library
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=dylib=spacevm");
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}

fn download_libs(lib: &Path) -> Result<()> {
    let dall = env::var("DOWNLOAD_ALL_LIBS")
        .map(|b| b == "true")
        .unwrap_or(false);
    for platform in PLATFORMS {
        let target = lib.join(platform);
        if target.join(LIB).exists() || (!dall && platform != PLATFORM) {
            continue;
        }

        fs::create_dir_all(&target)?;
        let mut curl = Command::new("curl");
        curl.current_dir(&target);
        curl.stdout(Stdio::piped());
        curl.arg("-L");
        curl.arg(format!("{LIB_BASE}/{LIB_NAME}-{platform}.tar.gz"));
        let output = curl.spawn()?;

        let mut tar = Command::new("tar");
        tar.current_dir(&target);
        tar.stdin(Stdio::from(
            output.stdout.expect("failed to download library"),
        ));
        tar.args(["xvzf", "-"]);
        tar.status()?;
    }

    Ok(())
}

mod platforms {
    #[cfg(target_os = "linux")]
    pub const LIB: &str = "libspacevm.so";
    #[cfg(target_os = "macos")]
    pub const LIB: &str = "libspacevm.dylib";

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    pub const PLATFORM: &str = "linux-amd64";

    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    pub const PLATFORM: &str = "linux-arm64";

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    pub const PLATFORM: &str = "macos-amd64";

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    pub const PLATFORM: &str = "macos-arm64";
}
