// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Removed grok.com workspace client.

use std::sync::Arc;

use serde::Deserialize;

use crate::auth::AuthManager;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    #[serde(default)]
    pub workspace_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub create_time: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct WsQuery {
    pub page_size: i64,
    pub page_token: Option<String>,
    pub query: Option<String>,
    pub kind: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ListWorkspacesPage {
    pub workspaces: Vec<Workspace>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum WsError {
    #[error("cloud workspaces were removed from this privacy build")]
    NoOauth,
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("request failed: {status}")]
    Http { status: u16 },
    #[error("parse error: {0}")]
    Parse(#[from] serde_json::Error),
}

pub struct WorkspacesClient {
    _auth: Arc<AuthManager>,
}

impl WorkspacesClient {
    pub fn new(auth: Arc<AuthManager>) -> Self {
        Self { _auth: auth }
    }

    pub(crate) async fn list_workspaces(
        &self,
        _q: &WsQuery,
    ) -> Result<ListWorkspacesPage, WsError> {
        Err(WsError::NoOauth)
    }
}
