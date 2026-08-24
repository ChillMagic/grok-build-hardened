// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Inert compatibility facade for the removed grok.com WebSocket relay.
//!
//! Upstream used this module for server-initiated remote control and live
//! session mirroring. The privacy build contains no WebSocket constructor,
//! handshake, reconnect loop, message serializer, or network sender.

use crate::auth::{AuthManager, GrokAuth, GrokComConfig};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

/// Unconstructable-in-practice marker retained for source compatibility.
#[derive(Clone)]
pub struct RelayConfig;

impl RelayConfig {
    /// Relay/cloud control is a compile-time-disabled capability.
    pub(crate) fn for_session(
        _session: &GrokAuth,
        _ctx: &GrokComConfig,
        _alpha_test_key: Option<String>,
        _auth_manager: Option<Arc<AuthManager>>,
    ) -> Option<Self> {
        None
    }
}

pub(crate) type FirstConnectCallback = Box<dyn FnOnce() + Send + 'static>;

/// Permanently stopped handle retained for callers that keep a slot for it.
pub struct RelayHandle {
    cancel: CancellationToken,
}

impl RelayHandle {
    pub fn stop(&self) {
        self.cancel.cancel();
    }

    pub fn is_running(&self) -> bool {
        false
    }
}

impl Drop for RelayHandle {
    fn drop(&mut self) {
        self.cancel.cancel();
    }
}

/// Returns a closed local channel. No task and no network connection are made.
pub fn spawn_relay_connection(
    _config: RelayConfig,
    _to_agent_tx: mpsc::UnboundedSender<String>,
    _parent_cancel: CancellationToken,
) -> (mpsc::UnboundedSender<String>, RelayHandle) {
    inert_connection()
}

/// Same fail-closed behavior as [`spawn_relay_connection`].
pub(crate) fn spawn_relay_connection_with_callback(
    _config: RelayConfig,
    _to_agent_tx: mpsc::UnboundedSender<String>,
    _parent_cancel: Option<CancellationToken>,
    _on_first_connect: Option<FirstConnectCallback>,
) -> (mpsc::UnboundedSender<String>, RelayHandle) {
    inert_connection()
}

fn inert_connection() -> (mpsc::UnboundedSender<String>, RelayHandle) {
    let (sender, receiver) = mpsc::unbounded_channel();
    drop(receiver);
    let cancel = CancellationToken::new();
    cancel.cancel();
    (sender, RelayHandle { cancel })
}
