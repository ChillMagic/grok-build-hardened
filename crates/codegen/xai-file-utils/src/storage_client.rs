//! No-network replacement for the cli-chat-proxy storage client.

use std::path::Path;

use anyhow::Result;

#[derive(Clone, Default)]
pub struct StorageClient;

impl StorageClient {
    pub fn new(_proxy_base_url: &str, _user_token: &str) -> Self {
        Self
    }

    pub fn storage_breaker_is_open(&self) -> bool {
        true
    }

    pub async fn download_blob(&self, _storage_path: &str, _dest: &Path) -> Result<()> {
        Err(crate::data_uploads_removed_error())
    }

    pub async fn upload(&self, _path: &str, _content: &[u8], _content_type: &str) -> Result<()> {
        Err(crate::data_uploads_removed_error())
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn storage_client_has_no_network_path() {
        let client = super::StorageClient::new("http://127.0.0.1:1", "unused");
        assert!(client.storage_breaker_is_open());
        assert!(client.upload("x", b"secret", "text/plain").await.is_err());
        assert!(
            client
                .download_blob("x", std::path::Path::new("unused"))
                .await
                .is_err()
        );
    }
}
