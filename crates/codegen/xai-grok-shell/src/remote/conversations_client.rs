// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Removed grok.com conversation client.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::auth::AuthManager;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conversation {
    #[serde(default)]
    pub conversation_id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub starred: bool,
    #[serde(default)]
    pub create_time: Option<String>,
    #[serde(default)]
    pub modify_time: Option<String>,
    #[serde(default)]
    pub workspaces: Vec<Workspace>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    #[serde(default)]
    pub workspace_id: String,
}

#[derive(Debug, Clone, Default)]
pub struct ConvQuery {
    pub page_size: i64,
    pub page_token: Option<String>,
    pub search_query: Option<String>,
    pub workspace_id: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ListConversationsPage {
    pub conversations: Vec<Conversation>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConversationBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred: Option<bool>,
}

#[derive(Debug, thiserror::Error)]
pub enum ConvError {
    #[error("cloud conversations were removed from this privacy build")]
    NoOauth,
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("request failed: {status}")]
    Http { status: u16 },
    #[error("parse error: {0}")]
    Parse(#[from] serde_json::Error),
}

pub struct ConversationsClient {
    _auth: Arc<AuthManager>,
}

impl ConversationsClient {
    pub fn new(auth: Arc<AuthManager>) -> Self {
        Self { _auth: auth }
    }

    pub async fn list_conversations(
        &self,
        _q: &ConvQuery,
    ) -> Result<ListConversationsPage, ConvError> {
        Err(ConvError::NoOauth)
    }

    pub async fn update_conversation(
        &self,
        _conversation_id: &str,
        _body: &UpdateConversationBody,
    ) -> Result<(), ConvError> {
        Err(ConvError::NoOauth)
    }

    pub(crate) async fn soft_delete_conversation(
        &self,
        _conversation_id: &str,
    ) -> Result<(), ConvError> {
        Err(ConvError::NoOauth)
    }
}
