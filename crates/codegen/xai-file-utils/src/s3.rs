// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Fail-closed S3 facade for the privacy build.
//!
//! Direct uploads, existence probes, and presigned PUT/GET generation are
//! intentionally deleted. Normal image/video generation still uses the xAI
//! media API; the optional customer-S3/ZDR video-output route is unavailable.

use std::path::Path;

#[derive(Clone)]
pub struct S3StaticCredentials {
    pub access_key_id: String,
    pub secret_access_key: String,
}

impl std::fmt::Debug for S3StaticCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("S3StaticCredentials")
            .field("access_key_id", &"[redacted]")
            .field("secret_access_key", &"[redacted]")
            .finish()
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn presign_put_url(
    _region: &str,
    _endpoint_url: Option<&str>,
    _creds: &S3StaticCredentials,
    _bucket: &str,
    _key: &str,
    _content_type: &str,
    _expires_in: std::time::Duration,
) -> anyhow::Result<String> {
    Err(crate::data_uploads_removed_error())
}

pub async fn presign_get_url(
    _region: &str,
    _endpoint_url: Option<&str>,
    _creds: &S3StaticCredentials,
    _bucket: &str,
    _key: &str,
    _expires_in: std::time::Duration,
) -> anyhow::Result<String> {
    Err(crate::data_uploads_removed_error())
}

#[allow(clippy::too_many_arguments)]
pub async fn upload_bytes(
    _bucket: &str,
    _object_path: &str,
    _content: &[u8],
    _content_type: &str,
    _region: &str,
    _credentials_content: Option<&str>,
    _credentials_file: Option<&str>,
    _endpoint_url: Option<&str>,
) -> anyhow::Result<String> {
    Err(crate::data_uploads_removed_error())
}

#[allow(clippy::too_many_arguments)]
pub async fn upload_file(
    _bucket: &str,
    _object_path: &str,
    _file_path: &Path,
    _content_type: &str,
    _region: &str,
    _credentials_content: Option<&str>,
    _credentials_file: Option<&str>,
    _endpoint_url: Option<&str>,
) -> anyhow::Result<String> {
    Err(crate::data_uploads_removed_error())
}

#[allow(clippy::too_many_arguments)]
pub async fn upload_stream<R: tokio::io::AsyncRead + Send + Sync + 'static>(
    _bucket: &str,
    _object_path: &str,
    _reader: R,
    _content_type: &str,
    _region: &str,
    _credentials_content: Option<&str>,
    _credentials_file: Option<&str>,
    _endpoint_url: Option<&str>,
) -> anyhow::Result<String> {
    Err(crate::data_uploads_removed_error())
}

#[derive(Debug, Clone)]
pub struct S3ExistsResponse {
    pub bucket: String,
    pub path: String,
    pub size: i64,
}

pub struct S3StorageClient {
    bucket: String,
}

impl S3StorageClient {
    pub fn bucket_name(&self) -> &str {
        &self.bucket
    }

    pub async fn new(
        bucket: String,
        _region: &str,
        _credentials_content: Option<&str>,
        _credentials_file: Option<&str>,
        _endpoint_url: Option<&str>,
    ) -> anyhow::Result<Self> {
        Ok(Self { bucket })
    }

    pub async fn check_exists(&self, _path: &str) -> Option<S3ExistsResponse> {
        None
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn presigning_is_removed() {
        let creds = super::S3StaticCredentials {
            access_key_id: "unused".into(),
            secret_access_key: "unused".into(),
        };
        assert!(
            super::presign_get_url(
                "unused",
                None,
                &creds,
                "unused",
                "unused",
                std::time::Duration::from_secs(1),
            )
            .await
            .is_err()
        );
    }
}
