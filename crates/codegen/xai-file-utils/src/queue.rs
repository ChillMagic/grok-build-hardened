// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! No-spill, no-worker upload queue facade for the privacy build.
//!
//! The upstream persistent queue, disk snapshots, retry worker, auth recovery,
//! and orphan replay are intentionally deleted. No constructor creates a
//! directory or starts a task.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::time::Duration;

use crate::{BlobCompression, TraceExportConfig};
use tokio::sync::{Notify, oneshot};

pub trait TraceExportSource: Send + Sync {
    fn resolve(&self) -> TraceExportConfig;

    fn resolve_async(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = TraceExportConfig> + Send + '_>> {
        Box::pin(std::future::ready(self.resolve()))
    }
}

pub const DEFAULT_MAX_AGE: Duration = Duration::ZERO;
pub const DEFAULT_AUTH_PARK_PROBE_INTERVAL: Duration = Duration::ZERO;

#[derive(Clone, Debug)]
pub struct UploadRetryPolicy {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub multiplier: f64,
    pub max_age: Duration,
    pub auth_park_probe_interval: Duration,
}

impl Default for UploadRetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 0,
            initial_delay: Duration::ZERO,
            max_delay: Duration::ZERO,
            multiplier: 0.0,
            max_age: Duration::ZERO,
            auth_park_probe_interval: Duration::ZERO,
        }
    }
}

pub const QUEUE_ITEM_SIDECAR_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct QueueItemSidecar {
    #[serde(default = "default_sidecar_schema_version")]
    pub schema_version: u32,
    pub session_id: String,
    pub turn_number: u64,
    pub gcs_path: String,
    pub content_type: String,
    pub artifact_name: String,
    pub enqueued_at: String,
    pub sha256: String,
}

fn default_sidecar_schema_version() -> u32 {
    QUEUE_ITEM_SIDECAR_SCHEMA_VERSION
}

#[derive(Debug)]
pub struct UploadCompletion {
    pub gcs_url: String,
    pub compression: BlobCompression,
    pub original_size: u64,
    pub stored_size: u64,
}

pub struct EnqueueResult {
    pub completion_rx: oneshot::Receiver<anyhow::Result<UploadCompletion>>,
    pub original_size: u64,
}

pub struct UploadQueueStats {
    pub pending: AtomicU64,
    pub pending_bytes: AtomicU64,
    pub inflight: AtomicU64,
    pub enqueued: AtomicU64,
    pub deduplicated: AtomicU64,
    pub uploaded: AtomicU64,
    pub failed: AtomicU64,
    pub circuit_breaker_trips: AtomicU64,
    pub circuit_breaker_active: AtomicBool,
    pub enqueue_fallbacks: AtomicU64,
    pub leaked_temp_files: AtomicU64,
    pub reference_stale: AtomicU64,
    pub auth_parked: AtomicU64,
    pub cleanup_orphan_mismatched: AtomicU64,
}

impl Default for UploadQueueStats {
    fn default() -> Self {
        Self::new()
    }
}

impl UploadQueueStats {
    pub fn new() -> Self {
        Self {
            pending: AtomicU64::new(0),
            pending_bytes: AtomicU64::new(0),
            inflight: AtomicU64::new(0),
            enqueued: AtomicU64::new(0),
            deduplicated: AtomicU64::new(0),
            uploaded: AtomicU64::new(0),
            failed: AtomicU64::new(0),
            circuit_breaker_trips: AtomicU64::new(0),
            circuit_breaker_active: AtomicBool::new(true),
            enqueue_fallbacks: AtomicU64::new(0),
            leaked_temp_files: AtomicU64::new(0),
            reference_stale: AtomicU64::new(0),
            auth_parked: AtomicU64::new(0),
            cleanup_orphan_mismatched: AtomicU64::new(0),
        }
    }

    pub fn set_transition_notify(&self, _notify: Arc<Notify>) {}
}

pub fn try_remove_temp(_path: &Path, _stats: Option<&UploadQueueStats>) {}

#[derive(Debug)]
pub struct QueueClosed;

impl std::fmt::Display for QueueClosed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(crate::DATA_UPLOADS_REMOVED_MESSAGE)
    }
}

impl std::error::Error for QueueClosed {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnqueueOutcome {
    Enqueued,
    FellBackToInline,
    Failed { reason: String },
    Deduplicated,
    Skipped { reason: String },
}

#[derive(Clone)]
pub struct UploadQueue {
    stats: Arc<UploadQueueStats>,
    pub client_version: Option<String>,
}

impl UploadQueue {
    /// Construct the inert queue without a resolver, worker, or filesystem path.
    pub fn disabled() -> Self {
        Self {
            stats: Arc::new(UploadQueueStats::new()),
            client_version: None,
        }
    }

    pub fn spawn(
        _grok_home: &Path,
        _resolver: Arc<dyn TraceExportSource>,
        _retry_policy: UploadRetryPolicy,
    ) -> Self {
        Self::disabled()
    }

    pub fn spawn_with_concurrency(
        grok_home: &Path,
        resolver: Arc<dyn TraceExportSource>,
        retry_policy: UploadRetryPolicy,
        _max_concurrent: usize,
    ) -> Self {
        Self::spawn(grok_home, resolver, retry_policy)
    }

    pub fn with_client_version(mut self, version: impl Into<String>) -> Self {
        self.client_version = Some(version.into());
        self
    }

    pub fn with_max_queue_bytes(self, _max_bytes: u64) -> Self {
        self
    }

    fn failed() -> EnqueueOutcome {
        EnqueueOutcome::Failed {
            reason: crate::DATA_UPLOADS_REMOVED_MESSAGE.to_owned(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn enqueue(
        &self,
        _content: &[u8],
        _gcs_path: &str,
        _content_type: &str,
        _artifact_name: &str,
        _session_id: &str,
        _turn_number: u64,
    ) -> anyhow::Result<()> {
        Err(crate::data_uploads_removed_error())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn enqueue_bytes_blocking(
        &self,
        _content: &[u8],
        _gcs_path: &str,
        _content_type: &str,
        _artifact_name: &str,
        _session_id: &str,
        _turn_number: u64,
    ) -> EnqueueOutcome {
        Self::failed()
    }

    pub fn enqueue_recovered(
        &self,
        _temp_path: &Path,
        _sidecar_path: &Path,
        _sidecar: &QueueItemSidecar,
    ) -> EnqueueOutcome {
        Self::failed()
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn enqueue_blocking(
        &self,
        _content: &[u8],
        _gcs_path: &str,
        _content_type: &str,
        _artifact_name: &str,
        _session_id: &str,
        _turn_number: u64,
    ) -> anyhow::Result<String> {
        Err(crate::data_uploads_removed_error())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn enqueue_file(
        &self,
        _source_path: &Path,
        _gcs_path: &str,
        _content_type: &str,
        _artifact_name: &str,
        _session_id: &str,
        _turn_number: u64,
    ) -> anyhow::Result<()> {
        Err(crate::data_uploads_removed_error())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn enqueue_file_blocking(
        &self,
        _source_path: &Path,
        _gcs_path: &str,
        _content_type: &str,
        _artifact_name: &str,
        _session_id: &str,
        _turn_number: u64,
        _compress: bool,
    ) -> anyhow::Result<EnqueueResult> {
        Err(crate::data_uploads_removed_error())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn enqueue_file_reference(
        &self,
        _source_path: &Path,
        _expected_sha256: &str,
        _gcs_path: &str,
        _content_type: &str,
        _artifact_name: &str,
        _session_id: &str,
        _turn_number: u64,
    ) -> anyhow::Result<EnqueueResult> {
        Err(crate::data_uploads_removed_error())
    }

    pub async fn wait_idle(&self, _timeout: Duration) -> usize {
        0
    }

    pub async fn drain(&self, _deadline: Duration) -> usize {
        0
    }

    pub fn stats(&self) -> &UploadQueueStats {
        &self.stats
    }

    pub fn stats_arc(&self) -> Arc<UploadQueueStats> {
        self.stats.clone()
    }

    pub fn cleanup_orphans(&self, _max_age: Duration) {}
}

pub const SIDECAR_SUFFIX: &str = ".meta.json";

pub fn sidecar_path_for(temp_path: &Path) -> PathBuf {
    let mut name = temp_path.as_os_str().to_owned();
    name.push(SIDECAR_SUFFIX);
    PathBuf::from(name)
}

pub fn temp_path_for_sidecar(sidecar: &Path) -> Option<PathBuf> {
    let name = sidecar.file_name()?.to_str()?;
    let stem = name.strip_suffix(SIDECAR_SUFFIX)?;
    Some(sidecar.with_file_name(stem))
}

pub fn last_orphans_cleaned() -> u64 {
    0
}

pub fn cleanup_orphaned_uploads(_grok_home: &Path, _max_age: Duration) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn queue_constructor_starts_no_worker_or_spill() {
        assert!(!crate::DATA_UPLOADS_COMPILED_IN);
        assert_eq!(super::DEFAULT_MAX_AGE, std::time::Duration::ZERO);
    }
}
