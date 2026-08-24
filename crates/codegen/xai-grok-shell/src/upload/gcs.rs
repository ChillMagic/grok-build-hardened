//! No-upload shell compatibility adapter.

use crate::auth::AuthManager;
use std::sync::Arc;
use xai_file_utils::gcs::StorageConfig;
use xai_file_utils::{TraceExportConfig, UploadMethod};

#[derive(Clone)]
pub(crate) struct TraceExportConfigWithAuth {
    inner: TraceExportConfig,
}

impl TraceExportConfigWithAuth {
    pub(crate) fn new(inner: TraceExportConfig, _auth_manager: Option<Arc<AuthManager>>) -> Self {
        Self { inner }
    }
}

impl StorageConfig for TraceExportConfigWithAuth {
    fn bucket_url(&self) -> &str {
        self.inner.bucket_url()
    }

    fn upload_method(&self) -> &UploadMethod {
        self.inner.upload_method()
    }
}

pub(crate) trait WithAuth {
    fn with_auth(&self, auth_manager: Option<Arc<AuthManager>>) -> TraceExportConfigWithAuth;
}

impl WithAuth for TraceExportConfig {
    fn with_auth(&self, auth_manager: Option<Arc<AuthManager>>) -> TraceExportConfigWithAuth {
        TraceExportConfigWithAuth::new(self.clone(), auth_manager)
    }
}

pub(crate) const SESSION_TRACES_BUCKET: Option<&str> = None;

pub(crate) async fn upload_to_auth_diagnostics(
    _log_bytes: &[u8],
    _user_id: &str,
    _upload_method: &crate::session::repo_changes::UploadMethod,
    _auth_manager: Arc<crate::auth::AuthManager>,
) {
}
