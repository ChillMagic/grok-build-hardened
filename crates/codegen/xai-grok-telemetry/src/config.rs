//! Inert telemetry configuration facade for the privacy build.
//!
//! Legacy fields remain deserializable so existing user configuration does
//! not break startup, but build-time values, environment overrides, endpoints,
//! tokens, and feature selections cannot activate telemetry.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TelemetryMode {
    #[default]
    Disabled,
    SessionMetrics,
    Enabled,
}

impl TelemetryMode {
    pub fn is_disabled(&self) -> bool {
        true
    }

    pub fn is_enabled(&self) -> bool {
        false
    }

    pub fn session_metrics_enabled(&self) -> bool {
        false
    }

    pub fn parse(raw: &str) -> Option<Self> {
        if raw.trim().is_empty() {
            None
        } else {
            Some(Self::Disabled)
        }
    }
}

impl std::fmt::Display for TelemetryMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("false")
    }
}

impl From<bool> for TelemetryMode {
    fn from(_value: bool) -> Self {
        Self::Disabled
    }
}

impl Serialize for TelemetryMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bool(false)
    }
}

impl<'de> Deserialize<'de> for TelemetryMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let _ = serde::de::IgnoredAny::deserialize(deserializer)?;
        Ok(Self::Disabled)
    }
}

pub fn env_telemetry_mode(_name: &str) -> Option<TelemetryMode> {
    Some(TelemetryMode::Disabled)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TelemetryConfig {
    pub enabled: Option<bool>,
    pub events_url: Option<String>,
    pub events_api_key: Option<String>,
    pub mixpanel_token: Option<String>,
    pub mixpanel_enabled: bool,
    pub trace_upload: Option<bool>,
    pub otel_enabled: Option<bool>,
    pub otel_metrics_exporter: Option<String>,
    pub otel_logs_exporter: Option<String>,
    pub otel_endpoint: Option<String>,
    pub otel_protocol: Option<String>,
    pub otel_certificate: Option<String>,
    pub otel_client_certificate: Option<String>,
    pub otel_client_key: Option<String>,
    pub otel_log_user_prompts: Option<bool>,
    pub otel_log_tool_details: Option<bool>,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            enabled: Some(false),
            events_url: None,
            events_api_key: None,
            mixpanel_token: None,
            mixpanel_enabled: false,
            trace_upload: Some(false),
            otel_enabled: Some(false),
            otel_metrics_exporter: None,
            otel_logs_exporter: None,
            otel_endpoint: None,
            otel_protocol: None,
            otel_certificate: None,
            otel_client_certificate: None,
            otel_client_key: None,
            otel_log_user_prompts: Some(false),
            otel_log_tool_details: Some(false),
        }
    }
}

impl TelemetryConfig {
    pub fn apply_env_overrides(&mut self) {
        *self = Self::default();
    }
}

pub fn deployment_id_from_key(key: &str) -> String {
    uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, key.as_bytes()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_modes_and_inputs_collapse_to_disabled() {
        for value in ["true", "session_metrics", "false", "arbitrary"] {
            assert_eq!(TelemetryMode::parse(value), Some(TelemetryMode::Disabled));
        }
        assert_eq!(TelemetryMode::from(true), TelemetryMode::Disabled);
        assert_eq!(env_telemetry_mode("IGNORED"), Some(TelemetryMode::Disabled));

        let cfg = TelemetryConfig::default();
        assert_eq!(cfg.enabled, Some(false));
        assert_eq!(cfg.trace_upload, Some(false));
        assert_eq!(cfg.otel_enabled, Some(false));
        assert!(!cfg.mixpanel_enabled);
    }
}
