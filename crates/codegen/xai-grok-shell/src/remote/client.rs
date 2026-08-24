// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Read-only/no-network compatibility facade for removed cloud-control and
//! remote session features.
//!
//! Session CRUD/share, subagent bundle delivery, remote settings, login
//! rollout flags, and remote model catalogs were removed as complete feature
//! implementations. Normal model inference uses the separately configured
//! sampling client and is not implemented in this module.

use crate::auth::GrokAuth;
use crate::session::export::{ExportedMessage, ExportedMetadata, ExportedSession};
use prod_mc_cli_chat_proxy_types::SubagentBundle;
use serde::{Deserialize, Serialize};

pub const REMOTE_BACKEND_COMPILED_IN: bool = false;
pub const REMOTE_BACKEND_REMOVED_MESSAGE: &str =
    "remote backend capability was removed from this privacy build";

fn removed<T>() -> Result<T, BackendError> {
    Err(BackendError::RequestFailed {
        status: 403,
        body: REMOTE_BACKEND_REMOVED_MESSAGE.to_owned(),
    })
}

pub fn share_url(_permission_id: &str) -> String {
    REMOTE_BACKEND_REMOVED_MESSAGE.to_owned()
}

pub async fn fetch_subagent_bundle(
    _cli_chat_proxy_base_url: &str,
    _auth_manager: Option<&std::sync::Arc<crate::auth::AuthManager>>,
    _deployment_key: Option<&str>,
    _alpha_test_key: Option<&str>,
) -> Result<SubagentBundle, BackendError> {
    removed()
}

#[derive(Debug)]
pub enum FetchedBundle {
    Archive(Vec<u8>),
    Legacy(SubagentBundle),
}

pub async fn fetch_bundle(
    _cli_chat_proxy_base_url: &str,
    _auth_manager: Option<&std::sync::Arc<crate::auth::AuthManager>>,
    _deployment_key: Option<&str>,
    _alpha_test_key: Option<&str>,
) -> Result<FetchedBundle, BackendError> {
    removed()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ShareResponse {
    pub permission_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LoadDataResponse {
    pub messages: Option<Vec<LoadedMessage>>,
    pub session: Option<SessionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LoadedMessage {
    pub id: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionInfo {
    pub session_id: String,
    pub title: Option<String>,
    pub cwd: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Request failed: {status} - {body}")]
    RequestFailed { status: u16, body: String },
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Session not found: {session_id}")]
    SessionNotFound { session_id: String },
    #[error("Hydration I/O error at {path}: {source}")]
    Hydration {
        path: std::path::PathBuf,
        source: std::io::Error,
    },
    #[error("Auth error: {0}")]
    Auth(String),
}

pub struct BackendClient {
    base_url: String,
    pub(crate) auth_manager: Option<std::sync::Arc<crate::auth::AuthManager>>,
}

impl Default for BackendClient {
    fn default() -> Self {
        Self::new()
    }
}

impl BackendClient {
    pub fn new() -> Self {
        Self {
            base_url: "privacy://remote-backend-disabled".to_owned(),
            auth_manager: None,
        }
    }

    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            auth_manager: None,
        }
    }

    pub(crate) fn with_auth_manager(
        mut self,
        manager: std::sync::Arc<crate::auth::AuthManager>,
    ) -> Self {
        self.auth_manager = Some(manager);
        self
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn share_session(
        &self,
        _session: &ExportedSession,
        _agent_id: &str,
    ) -> Result<String, BackendError> {
        removed()
    }

    pub async fn upsert_session(
        &self,
        _session_id: &str,
        _metadata: &ExportedMetadata,
        _agent_id: &str,
    ) -> Result<(), BackendError> {
        removed()
    }

    pub(crate) async fn save_session_data(
        &self,
        _session_id: &str,
        _messages: &[ExportedMessage],
        _metadata: Option<&ExportedMetadata>,
    ) -> Result<(), BackendError> {
        removed()
    }

    pub async fn list_sessions(&self) -> Result<Vec<SessionInfo>, BackendError> {
        removed()
    }

    pub(crate) async fn load_session_data(
        &self,
        _session_id: &str,
    ) -> Result<LoadDataResponse, BackendError> {
        removed()
    }

    pub(crate) async fn create_share_link(
        &self,
        _session_id: &str,
    ) -> Result<ShareResponse, BackendError> {
        removed()
    }

    pub(crate) async fn delete_session_data(&self, _session_id: &str) -> Result<(), BackendError> {
        removed()
    }
}

#[derive(Debug)]
#[must_use]
#[non_exhaustive]
pub enum SettingsFetch {
    Fetched(Box<crate::util::config::RemoteSettings>),
    Rejected,
    Retry,
}

impl SettingsFetch {
    pub fn into_option(self) -> Option<crate::util::config::RemoteSettings> {
        None
    }
}

pub fn fetch_settings_blocking(
    _cli_chat_proxy_base_url: &str,
    _auth: &GrokAuth,
    _alpha_test_key: Option<&str>,
) -> SettingsFetch {
    SettingsFetch::Rejected
}

pub async fn fetch_login_device_flow(_cli_chat_proxy_base_url: &str) -> Option<bool> {
    None
}

pub(crate) const DEFAULT_CONTEXT_WINDOW: u64 = 256_000;

pub(crate) fn models_list_url(
    _endpoints: &crate::agent::config::EndpointsConfig,
    _fetch_auth: crate::agent::models::ModelFetchAuth,
) -> String {
    "privacy://remote-model-catalog-disabled".to_owned()
}

pub struct FetchModelsResult {
    pub models: Vec<crate::agent::config::ModelEntryConfig>,
    pub etag: Option<String>,
}

pub(crate) fn fetch_models_blocking(
    _endpoints: &crate::agent::config::EndpointsConfig,
    _auth: Option<&GrokAuth>,
    _fetch_auth: crate::agent::models::ModelFetchAuth,
) -> Result<FetchModelsResult, BackendError> {
    removed()
}

pub(crate) fn parse_remote_model_value(
    _value: &serde_json::Value,
    _default_base_url: &str,
) -> Option<crate::agent::config::ModelEntryConfig> {
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn cloud_backend_is_permanently_inert() {
        assert!(!super::REMOTE_BACKEND_COMPILED_IN);
        assert_eq!(
            super::models_list_url(
                &crate::agent::config::EndpointsConfig::default(),
                crate::agent::models::ModelFetchAuth::Session,
            ),
            "privacy://remote-model-catalog-disabled"
        );
    }
}
