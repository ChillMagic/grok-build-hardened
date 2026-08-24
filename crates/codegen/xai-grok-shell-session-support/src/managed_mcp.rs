//! Inert facade for the removed vendor-managed MCP gateway.
//!
//! Local, project, user, and plugin MCP servers are implemented elsewhere and
//! remain available. This module deliberately contains no HTTP client, catalog
//! fetcher, or tool-call sender.

use std::collections::HashSet;
use std::sync::Arc;

pub enum GatewayToolCatalogCache {
    NotFetched,
    Fetching(u64),
    Ready(GatewayToolCatalog),
}

pub struct ManagedMcpState {
    pub gateway_tools_active: bool,
    pub gateway_tool_epoch: u64,
    pub gateway_tool_cache: GatewayToolCatalogCache,
    pub gateway_tool_fetch_notify: Arc<tokio::sync::Notify>,
    pub gateway_tool_connectors_seen: HashSet<String>,
}

impl Default for ManagedMcpState {
    fn default() -> Self {
        Self {
            gateway_tools_active: false,
            gateway_tool_epoch: 0,
            gateway_tool_cache: GatewayToolCatalogCache::NotFetched,
            gateway_tool_fetch_notify: Arc::new(tokio::sync::Notify::new()),
            gateway_tool_connectors_seen: HashSet::new(),
        }
    }
}

impl ManagedMcpState {
    pub fn enable_gateway_tools(&mut self) -> u64 {
        self.disable_gateway_tools();
        self.gateway_tool_epoch
    }

    pub fn start_gateway_tool_fetch(&mut self) -> Option<u64> {
        None
    }

    pub fn complete_gateway_tool_fetch(
        &mut self,
        _epoch: u64,
        _catalog: GatewayToolCatalog,
    ) -> bool {
        self.disable_gateway_tools();
        false
    }

    pub fn fail_gateway_tool_fetch(&mut self, _epoch: u64) {
        self.disable_gateway_tools();
    }

    pub fn disable_gateway_tools(&mut self) {
        self.gateway_tools_active = false;
        self.gateway_tool_epoch = self.gateway_tool_epoch.wrapping_add(1);
        self.gateway_tool_cache = GatewayToolCatalogCache::NotFetched;
        self.gateway_tool_fetch_notify.notify_waiters();
    }
}

pub type ManagedMcpStateHandle = Arc<tokio::sync::Mutex<ManagedMcpState>>;

#[derive(Debug, Clone, serde::Serialize)]
pub struct GatewayToolCallRequest {
    pub call_id: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct GatewayToolCallResponse {
    pub result: serde_json::Value,
    #[serde(default)]
    pub connectors_needing_reauth: Vec<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct GatewayToolCatalog {
    #[serde(default)]
    pub tools: Vec<GatewayTool>,
    #[serde(default)]
    pub total_tools: u32,
    #[serde(default)]
    pub connectors_needing_reauth: Vec<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct GatewayTool {
    pub connector_id: String,
    pub connector_name: String,
    pub tool_id: String,
    pub tool_name: String,
    pub call_id: String,
    pub description: String,
    pub json_schema: serde_json::Value,
}

impl GatewayTool {
    pub fn qualified_name(&self) -> String {
        format!("{}__{}", self.connector_id, self.tool_id)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ManagedMcpFetchError {
    #[error("HTTP {status}: {message}")]
    Status { status: u16, message: String },
    #[error("transport: {0}")]
    Transport(String),
    #[error("no auth token available")]
    NoAuth,
}

fn disabled<T>() -> Result<T, ManagedMcpFetchError> {
    Err(ManagedMcpFetchError::Status {
        status: 403,
        message: "vendor-managed MCP gateway disabled by privacy build".to_owned(),
    })
}

pub async fn call_gateway_tool(
    _proxy_base_url: &str,
    _auth_key: &str,
    _call_id: &str,
    _arguments: serde_json::Value,
) -> Result<GatewayToolCallResponse, ManagedMcpFetchError> {
    disabled()
}

pub async fn fetch_gateway_tool_catalog(
    _proxy_base_url: &str,
    _auth_key: &str,
) -> Result<GatewayToolCatalog, ManagedMcpFetchError> {
    disabled()
}

pub async fn invalidate_gateway_tool_cache(handle: &ManagedMcpStateHandle) {
    handle.lock().await.disable_gateway_tools();
}

pub async fn get_or_fetch_gateway_tool_catalog(
    handle: &ManagedMcpStateHandle,
    _proxy_url: &str,
    _auth_key: Option<&str>,
) -> Option<GatewayToolCatalog> {
    handle.lock().await.disable_gateway_tools();
    None
}

pub fn normalize_url(url: &str) -> String {
    url.trim_end_matches('/').to_owned()
}
