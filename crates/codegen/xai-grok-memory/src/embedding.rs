//! Local-only memory embedding facade for the privacy build.
//!
//! The API embedding client, credential middleware, request construction,
//! retries, and response decoder are removed. Memory search falls back to
//! SQLite FTS and never sends memory text or search queries to a service.

use async_trait::async_trait;

#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    async fn embed_batch(
        &self,
        texts: &[&str],
    ) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>>;

    fn model_name(&self) -> &str;

    fn dimensions(&self) -> usize;
}

/// Compatibility type which cannot be constructed in this build.
pub struct ApiEmbeddingProvider;

impl ApiEmbeddingProvider {
    pub fn from_session(
        _config: &xai_grok_config_types::MemoryEmbeddingConfig,
        _proxy_base_url: String,
        _auth_key: String,
    ) -> Option<Self> {
        None
    }
}

#[async_trait]
impl EmbeddingProvider for ApiEmbeddingProvider {
    async fn embed_batch(
        &self,
        _texts: &[&str],
    ) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
        Err("remote memory embeddings are removed in the privacy build".into())
    }

    fn model_name(&self) -> &str {
        "disabled"
    }

    fn dimensions(&self) -> usize {
        0
    }
}

#[cfg(any(test, feature = "test-support"))]
pub struct MockEmbeddingProvider {
    pub dimensions: usize,
}

#[cfg(any(test, feature = "test-support"))]
#[async_trait]
impl EmbeddingProvider for MockEmbeddingProvider {
    async fn embed_batch(
        &self,
        texts: &[&str],
    ) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
        Ok(texts
            .iter()
            .map(|text| {
                let hash = blake3::hash(text.as_bytes());
                let bytes = hash.as_bytes();
                (0..self.dimensions)
                    .map(|i| bytes[i % 32] as f32 / 255.0)
                    .collect()
            })
            .collect())
    }

    fn model_name(&self) -> &str {
        "mock-embedding"
    }

    fn dimensions(&self) -> usize {
        self.dimensions
    }
}
