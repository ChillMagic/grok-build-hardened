// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Compatibility types for local data handling in the privacy build.

/// This fork is intentionally compiled without passive/background data uploads.
///
/// This is a compile-time invariant rather than a setting: configuration,
/// environment variables, command-line flags, and remote responses must never
/// be able to re-enable repository, session, trace, feedback, or telemetry
/// storage uploads in the privacy build.
pub const DATA_UPLOADS_COMPILED_IN: bool = false;

/// Stable error text returned by every removed storage/upload entry point.
pub const DATA_UPLOADS_REMOVED_MESSAGE: &str =
    "data upload capability was removed from this privacy build";

/// Construct the fail-closed error returned by removed upload APIs.
pub fn data_uploads_removed_error() -> anyhow::Error {
    anyhow::anyhow!(DATA_UPLOADS_REMOVED_MESSAGE)
}
pub mod gcs;
pub mod queue;
pub mod s3;
pub mod storage_client;
pub mod trace_context;
pub mod upload_config;
pub mod workspace_classifier;
pub use upload_config::*;
/// Compute SHA256 hash of content as a hex string.
pub fn sha256_hex(content: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(content);
    format!("{:x}", hasher.finalize())
}
/// Compute SHA256 hash of a file by streaming, without loading entire file into memory.
/// If `max_bytes` is set (> 0), only hash up to that many bytes.
pub fn sha256_hex_from_file(
    path: &std::path::Path,
    max_bytes: Option<u64>,
) -> std::io::Result<String> {
    use sha2::{Digest, Sha256};
    use std::io::Read;
    let file = std::fs::File::open(path)?;
    let mut reader: Box<dyn Read> = if let Some(limit) = max_bytes {
        Box::new(file.take(limit))
    } else {
        Box::new(file)
    };
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}
