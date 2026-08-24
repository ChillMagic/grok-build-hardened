// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Removed remote session synchronization worker.

use crate::remote::client::BackendClient;
use crate::session::export::ExportedMetadata;
use agent_client_protocol as acp;

#[derive(Clone, Default)]
pub struct RemoteSync;

impl RemoteSync {
    #[cfg(test)]
    pub(crate) fn test_observer() -> (
        Self,
        tokio::sync::mpsc::UnboundedReceiver<acp::SessionNotification>,
    ) {
        let (_tx, rx) = tokio::sync::mpsc::unbounded_channel();
        (Self, rx)
    }

    pub(crate) fn new(
        _session_id: String,
        _metadata: ExportedMetadata,
        _client: BackendClient,
    ) -> Self {
        Self
    }

    pub fn queue(&self, _notification: acp::SessionNotification) {}
    pub fn flush(&self) {}
    pub fn set_title(&self, _title: String) {}
    pub fn set_manual_title(&self, _title: String) {}
    pub fn clear_title(&self) {}
    pub(crate) fn set_model_id(&self, _model_id: String) {}
}
