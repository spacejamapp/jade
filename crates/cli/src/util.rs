//! Utility functions for the PVM CLI

use anyhow::Result;
use std::{
    env, fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

/// Build the PVM blob
///
/// NOTE: this is used for the build script of services
pub fn build(package: &str, path: Option<String>) -> Result<()> {
    let target = env::var("TARGET")?;
    if target.contains("polkavm") {
        return Ok(());
    }

    // Build the service
    let target = etc::find_up("target")?;
    let jam = target.join("jam");
    let current = path
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().expect("Unable to get current directory"));
    let binary = jam.join(package).join(format!("{package}.jam"));
    let rebuild = if !binary.exists() {
        true
    } else {
        let modified = fs::metadata(&binary)?.modified()?;
        check_modified(&current, modified)?
    };

    if rebuild {
        let mut build = crate::cmd::Build::default();
        build.target = Some(jam.join(package));
        build.run()?;
    }

    // copy service to OUT_DIR
    let service = PathBuf::from(env::var("OUT_DIR")?).join("service.jam");
    println!("Copying service to OUT_DIR: {}", service.display());
    fs::copy(&binary, &service)?;
    Ok(())
}

/// Check if any Rust source files have been modified after the given time
fn check_modified(dir: &Path, since: SystemTime) -> Result<bool> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            if check_modified(&path, since)? {
                return Ok(true);
            }
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs")
            && metadata.modified()? > since
        {
            return Ok(true);
        }
    }

    Ok(false)
}
