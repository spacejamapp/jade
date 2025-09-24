//! Link spacevm to this sys library

use std::{env, fs, path::PathBuf, process::Command};

fn main() -> std::io::Result<()> {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    Command::new("curl")
        .args(&[
            "-L",
            "https://github.com/spacejamapp/specjam/releases/download/0.7.0-pre.9/libspacevm.a",
            "-o",
            "libspacevm.a",
        ])
        .status()?;

    let lib = out.join("libspacevm.a");
    #[cfg(target_os = "linux")]
    if !Command::new("ranlib").arg(&lib).status()?.success() {
        panic!("ranlib failed");
    }

    fs::rename("libspacevm.a", &lib)?;
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=spacevm");
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
