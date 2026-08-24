// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Local compatibility helpers for the privacy build.
//!
//! Server bundle payload parsing, archive extraction, manifest installation,
//! overwrite, and pruning are removed. The shell never treats a file as
//! server-managed and never installs remote agents, skills, roles, personas,
//! or workflows.

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

pub fn bundled_root() -> PathBuf {
    xai_grok_config::grok_home().join("bundled-disabled")
}

pub fn checksum_file(path: &Path) -> Result<String> {
    let bytes =
        std::fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

pub fn is_managed_bundle_file(_root: &Path, _relative_path: &str) -> bool {
    false
}
