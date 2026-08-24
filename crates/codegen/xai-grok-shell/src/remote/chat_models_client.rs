//! Removed server-delivered chat-mode catalog.

use std::sync::Arc;

use serde::Deserialize;

use crate::auth::AuthManager;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mode {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub badge_text: Option<String>,
    #[serde(default)]
    pub availability: ModeAvailability,
    #[serde(default)]
    pub icon_hint: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Mode {
    pub fn is_available(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModeAvailability {
    #[serde(default)]
    pub available: Option<serde_json::Value>,
    #[serde(default)]
    pub unavailable: Option<serde_json::Value>,
    #[serde(default)]
    pub requires_upgrade: Option<serde_json::Value>,
    #[serde(default)]
    pub coming_soon: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListModesResponse {
    #[serde(default)]
    pub modes: Vec<Mode>,
    #[serde(default)]
    pub default_mode_id: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ChatModelsError {
    #[error("server-delivered chat modes were removed from this privacy build")]
    NoAuth,
    #[error("request timed out")]
    Timeout,
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("request failed: {status}")]
    Http { status: u16 },
    #[error("parse error: {0}")]
    Parse(#[from] serde_json::Error),
}

pub struct ChatModelsClient {
    _auth: Arc<AuthManager>,
}

impl ChatModelsClient {
    pub fn new(auth: Arc<AuthManager>) -> Self {
        Self { _auth: auth }
    }

    pub(crate) async fn list_modes(
        &self,
        _locale: &str,
    ) -> Result<ListModesResponse, ChatModelsError> {
        Err(ChatModelsError::NoAuth)
    }
}
