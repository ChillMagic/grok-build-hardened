// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Fail-closed storage facade for the privacy build.
//!
//! The GCS, proxy-storage, signed-URL, and multipart implementations are
//! intentionally deleted.  These signatures remain only so inactive upstream
//! call sites compile and all attempts fail before reading payload bytes.

use crate::UploadMethod;
use std::path::Path;

pub const MULTIPART_UPLOAD_THRESHOLD: u64 = u64::MAX;

impl StorageConfig for crate::TraceExportConfig {
    fn bucket_url(&self) -> &str {
        self.bucket_url.as_deref().unwrap_or("")
    }

    fn upload_method(&self) -> &UploadMethod {
        &self.upload_method
    }
}

pub trait StorageConfig {
    fn bucket_url(&self) -> &str;
    fn upload_method(&self) -> &UploadMethod;
}

pub async fn upload_bytes<C: StorageConfig>(
    _config: &C,
    _object_path: &str,
    _content: &[u8],
    _content_type: &str,
) -> anyhow::Result<String> {
    Err(crate::data_uploads_removed_error())
}

pub async fn upload_bytes_signed<C: StorageConfig>(
    _config: &C,
    _object_path: &str,
    _content: &[u8],
    _content_type: &str,
) -> anyhow::Result<String> {
    Err(crate::data_uploads_removed_error())
}

pub async fn upload_file<C: StorageConfig>(
    _config: &C,
    _object_path: &str,
    _file_path: &Path,
    _content_type: &str,
) -> anyhow::Result<String> {
    Err(crate::data_uploads_removed_error())
}

pub async fn upload_stream<C: StorageConfig, R>(
    _config: &C,
    _object_path: &str,
    _reader: R,
    _content_type: &str,
) -> anyhow::Result<String>
where
    R: tokio::io::AsyncRead + Send + Sync + 'static,
{
    Err(crate::data_uploads_removed_error())
}

#[cfg(test)]
mod tests {
    #[test]
    fn gcs_uploads_are_removed() {
        assert!(!crate::DATA_UPLOADS_COMPILED_IN);
    }
}
