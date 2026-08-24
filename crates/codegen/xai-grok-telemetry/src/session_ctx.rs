//! No-emission telemetry session facade for the privacy build.

use std::sync::Arc;

use serde::Serialize;

use crate::events::TelemetryEvent;

#[derive(Clone)]
pub struct TelemetryCtx {
    pub session_id: String,
    pub prompt_index: Arc<tokio::sync::Mutex<usize>>,
    pub prompt_id: Arc<parking_lot::Mutex<Option<String>>>,
}

impl TelemetryCtx {
    pub fn new(session_id: String, prompt_index: Arc<tokio::sync::Mutex<usize>>) -> Self {
        Self {
            session_id,
            prompt_index,
            prompt_id: Arc::new(parking_lot::Mutex::new(None)),
        }
    }
}

pub fn begin_prompt_id() {}

pub(crate) const SESSION_ID_FIELD: &str = "session_id";

pub async fn with_session_ctx<F: std::future::Future>(_ctx: TelemetryCtx, fut: F) -> F::Output {
    fut.await
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumCount)]
pub enum EmitterOrigin {
    Shell,
    Workspace,
}

impl EmitterOrigin {
    pub const ALL: [EmitterOrigin; 2] = [EmitterOrigin::Shell, EmitterOrigin::Workspace];

    pub fn event_prefix(self) -> &'static str {
        match self {
            EmitterOrigin::Shell => "grok-shell-",
            EmitterOrigin::Workspace => "grok-workspace-",
        }
    }
}

pub fn log_event<T: TelemetryEvent>(_data: T) {}

pub fn log_event_dual<T: TelemetryEvent>(_internal_enabled: bool, _data: T) {}

pub fn log_session_event<T: TelemetryEvent>(_data: T) {}

pub fn log_session_event_with_origin<T: TelemetryEvent>(_origin: EmitterOrigin, _data: T) {}

pub fn emit_event<T: Serialize + Send + 'static>(_event_suffix: impl Into<String>, _data: T) {}

pub const CLI_DRAIN: std::time::Duration = std::time::Duration::ZERO;

pub async fn drain_at_session_exit() {}

pub async fn drain_at_process_exit() {}

pub async fn drain_pending(_timeout: std::time::Duration) {}

pub fn emit_event_with_origin<T: Serialize + Send + 'static>(
    _origin: EmitterOrigin,
    _event_suffix: impl Into<String>,
    _data: T,
) {
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn session_facade_has_no_pending_network_work() {
        drain_pending(std::time::Duration::ZERO).await;
        assert_eq!(CLI_DRAIN, std::time::Duration::ZERO);
    }
}
