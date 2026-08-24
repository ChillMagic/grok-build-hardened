// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Offline-only build script for the privacy build.
//!
//! Upstream downloaded ripgrep during release builds. This build never opens
//! the network: an explicitly supplied local binary may be embedded; otherwise
//! Grok Build uses `rg` from `PATH` at runtime.

use std::env;
use std::fs;
use std::path::PathBuf;

const RG_VER: &str = "15.0.0";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-env-changed=GROK_SHELL_BUNDLE_RG_PATH");
    println!("cargo:rustc-check-cfg=cfg(bundle_rg)");

    let Some(path) = env::var_os("GROK_SHELL_BUNDLE_RG_PATH") else {
        return Ok(());
    };

    let gen_dir = PathBuf::from(env::var("OUT_DIR")?);
    fs::create_dir_all(&gen_dir)?;
    let dest = gen_dir.join(format!("rg-{RG_VER}-override.bin"));
    fs::copy(&path, &dest).map_err(|error| {
        format!(
            "failed to copy local GROK_SHELL_BUNDLE_RG_PATH {} to {}: {error}",
            PathBuf::from(path).display(),
            dest.display()
        )
    })?;

    println!("cargo:rustc-cfg=bundle_rg");
    println!("cargo:rustc-env=GROK_SHELL_RG_VER={RG_VER}");
    println!(
        "cargo:rustc-env=GROK_SHELL_RG_GEN_DIR={}",
        gen_dir.display()
    );
    println!("cargo:rustc-env=GROK_SHELL_RG_TARGET=override");
    Ok(())
}
