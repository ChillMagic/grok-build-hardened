//! Offline-only search-tool bundling for the privacy build.
//!
//! Nothing is downloaded. Explicit local paths may be embedded; when absent,
//! runtime resolution falls back to the user's `PATH`.

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    bundle_local("rg", "RG", "15.0.0")?;
    bundle_local("bfs", "BFS", "4.1")?;
    bundle_local("ugrep", "UGREP", "7.7.0")?;
    Ok(())
}

fn bundle_local(
    name: &str,
    name_upper: &str,
    version: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let path_variable = format!("GROK_TOOLS_BUNDLE_{name_upper}_PATH");
    println!("cargo:rerun-if-env-changed={path_variable}");
    println!("cargo:rustc-check-cfg=cfg(bundle_{name})");

    let Some(source) = env::var_os(&path_variable) else {
        return Ok(());
    };

    let output_dir = PathBuf::from(env::var("OUT_DIR")?).join(format!("bundle-{name}"));
    fs::create_dir_all(&output_dir)?;
    let destination = output_dir.join(format!("{name}-{version}-override.bin"));
    fs::copy(&source, &destination).map_err(|error| {
        format!(
            "failed to copy local {path_variable} {} to {}: {error}",
            PathBuf::from(source).display(),
            destination.display()
        )
    })?;

    println!("cargo:rustc-cfg=bundle_{name}");
    println!("cargo:rustc-env=GROK_TOOLS_{name_upper}_VER={version}");
    println!("cargo:rustc-env=GROK_TOOLS_{name_upper}_TARGET=override");
    Ok(())
}
