// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Inert compatibility facade for removed cloud relay session mirroring.

use crate::agent::relay::RelayConfig;
use crate::relay::types::AgentType;
use agent_client_protocol as acp;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::watch;

pub(crate) fn build_share_url(session_id: &str) -> String {
    format!("privacy://relay-disabled/{session_id}")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
}

impl ConnectionState {
    pub fn is_connected(&self) -> bool {
        false
    }

    pub fn status_indicator(&self) -> &'static str {
        "📡 ✗"
    }
}

impl std::fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("disconnected")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelaySyncState {
    #[serde(default)]
    pub last_synced_event_id: Option<String>,
    #[serde(default)]
    pub last_synced_at: Option<u64>,
    #[serde(default)]
    pub relay_session_id: Option<String>,
    #[serde(default)]
    pub synced_count: u64,
}

#[derive(Debug, Clone)]
pub struct SyncStatus {
    pub has_sync_state: bool,
    pub synced_count: u64,
    pub last_synced_event_id: Option<String>,
    pub last_synced_at: Option<u64>,
}

impl RelaySyncState {
    /// Ignore state produced by an upstream build; it cannot authorize sync.
    pub fn load(_session_dir: &std::path::Path) -> Self {
        Self::default()
    }

    /// Refuse to persist a marker claiming that cloud sync occurred.
    pub fn save(&self, _session_dir: &std::path::Path) -> std::io::Result<()> {
        Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            crate::privacy_build::REMOVED_MESSAGE,
        ))
    }

    pub fn exists(_session_dir: &std::path::Path) -> bool {
        false
    }

    pub(crate) fn update_cursor(&mut self, _event_id: String) {}

    pub fn get_sync_status(_session_dir: &std::path::Path) -> SyncStatus {
        SyncStatus {
            has_sync_state: false,
            synced_count: 0,
            last_synced_event_id: None,
            last_synced_at: None,
        }
    }
}

pub type StatusCallback = Arc<dyn Fn(ConnectionState) + Send + Sync + 'static>;

/// Locally inert handle. Notifications are dropped without serialization.
pub struct RelaySync {
    session_id: String,
    agent_type: AgentType,
    connection_state_rx: watch::Receiver<ConnectionState>,
}

impl RelaySync {
    pub fn new(
        session_id: String,
        _config: RelayConfig,
        agent_type: AgentType,
        _session_dir: Option<PathBuf>,
        status_cb: Option<StatusCallback>,
    ) -> RelaySync {
        let (_state_tx, state_rx) = watch::channel(ConnectionState::Disconnected);
        if let Some(callback) = status_cb {
            callback(ConnectionState::Disconnected);
        }
        RelaySync {
            session_id,
            agent_type,
            connection_state_rx: state_rx,
        }
    }

    pub fn queue(&self, _notification: acp::SessionNotification) {}

    pub fn flush(&self) {}

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn agent_type(&self) -> AgentType {
        self.agent_type
    }

    pub(crate) fn connection_state(&self) -> ConnectionState {
        ConnectionState::Disconnected
    }

    pub fn is_connected(&self) -> bool {
        false
    }

    pub fn pending_count(&self) -> usize {
        0
    }

    pub(crate) fn subscribe_state(&self) -> watch::Receiver<ConnectionState> {
        self.connection_state_rx.clone()
    }
}
