//! No-network telemetry facade for the privacy build.
//!
//! The upstream Mixpanel and product-event sender is intentionally deleted.

use crate::config::{TelemetryConfig, TelemetryMode};
use crate::http::OriginClientInfo;

pub type Metadata = serde_json::Map<String, serde_json::Value>;

#[derive(Clone, Debug, Default)]
pub struct TelemetryClient;

impl TelemetryClient {
    #[allow(clippy::too_many_arguments)]
    pub fn from_config<T>(
        _config: TelemetryConfig,
        _mode: TelemetryMode,
        _user_id: Option<String>,
        _team_id: Option<String>,
        _deployment_key: Option<String>,
        _origin_client: Option<OriginClientInfo>,
        _shell_version: String,
        _subscription_tier: Option<String>,
        _http_client: T,
    ) -> Self {
        Self
    }
}

pub fn is_enabled() -> bool {
    false
}

pub fn is_session_metrics_enabled() -> bool {
    false
}

#[derive(Debug, Clone, Default)]
pub struct UserContext {
    pub country: String,
    pub language: String,
    pub timestamp: String,
}

impl UserContext {
    pub fn collect() -> Self {
        Self::default()
    }
}

pub const RESERVED_EVENT_KEYS: &[&str] = &[];

pub async fn track(_event_name: &str, _request_id: &str, _ctx: &UserContext, _metadata: Metadata) {}

pub fn current_mode() -> Option<TelemetryMode> {
    None
}

pub fn sync_profile() {}

#[allow(clippy::too_many_arguments)]
pub fn init<T>(
    _config: TelemetryConfig,
    _mode: TelemetryMode,
    _user_id: Option<String>,
    _team_id: Option<String>,
    _deployment_key: Option<String>,
    _origin_client: Option<OriginClientInfo>,
    _shell_version: String,
    _subscription_tier: Option<String>,
    _http_client: T,
) {
}

#[allow(clippy::too_many_arguments)]
pub fn init_if_needed<T>(
    _config: TelemetryConfig,
    _mode: TelemetryMode,
    _user_id: Option<String>,
    _team_id: Option<String>,
    _deployment_key: Option<String>,
    _origin_client: Option<OriginClientInfo>,
    _shell_version: String,
    _subscription_tier: Option<String>,
    _http_client: T,
) {
}

#[cfg(test)]
mod tests {
    #[test]
    fn telemetry_is_compile_time_inert() {
        assert!(!super::is_enabled());
        assert!(!super::is_session_metrics_enabled());
        assert!(super::current_mode().is_none());
    }
}
