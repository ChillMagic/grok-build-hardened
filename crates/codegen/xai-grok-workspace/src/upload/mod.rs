//! Privacy-build compatibility facade.
//!
//! The upstream workspace upload implementation (proxy credentials, queue
//! workers, retry/sampling logic, and tool-state artifact export) is removed
//! as a whole. These inert symbols only keep internal call sites source
//! compatible; none can create a client, queue worker, spill file, or request.

pub(crate) mod environment;

use std::sync::Arc;

use environment::WorkspaceIdentity;
use xai_computer_hub_sdk::auth::AuthProvider;
use xai_file_utils::queue::{TraceExportSource, UploadQueue};
use xai_file_utils::{TraceExportConfig, UploadMethod};

pub(crate) fn record_upload_outcome(_phase: &str, _outcome: &str) {}

pub(crate) fn record_upload_failed(_phase: &str, _error_category: &str) {}

pub(crate) fn record_upload_skipped(_phase: &str, _skip_reason: &str) {}

pub(crate) fn init_metrics() {}

pub(crate) fn spawn_queue_stats_sampler(
    _queue: Arc<UploadQueue>,
    _interval: std::time::Duration,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async {})
}

/// Inert replacement for the removed proxy-backed storage configuration.
pub(crate) struct ProxyStorageConfig;

impl ProxyStorageConfig {
    pub(crate) fn new(
        _auth: Arc<dyn AuthProvider>,
        _api_base_url: String,
        _identity: WorkspaceIdentity,
    ) -> Self {
        Self
    }
}

/// Inert resolver retained only for source compatibility.
pub(crate) struct WorkspaceTraceExportSource;

impl WorkspaceTraceExportSource {
    pub(crate) fn new(_proxy_storage_config: Arc<ProxyStorageConfig>) -> Self {
        Self
    }
}

impl TraceExportSource for WorkspaceTraceExportSource {
    fn resolve(&self) -> TraceExportConfig {
        TraceExportConfig {
            bucket_url: None,
            service_account_key: None,
            upload_method: UploadMethod::Direct {
                service_account_key: None,
            },
            prefix_dir: None,
            gcs_prefix: None,
            absolute_paths: false,
            archive_name_override: None,
        }
    }
}

pub(crate) async fn upload_tool_state_queued(
    _state_bytes: Vec<u8>,
    _session_id: String,
    _turn_number: u64,
    _upload_queue: Arc<UploadQueue>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err(xai_file_utils::data_uploads_removed_error().into())
}
