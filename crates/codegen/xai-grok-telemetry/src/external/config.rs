// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Compile-time-disabled external telemetry configuration facade.
//!
//! Endpoint parsing, environment activation, credential loading, and exporter
//! selection were removed as a whole file.  The data shapes remain only so
//! callers compiled against the upstream API do not need privacy-specific
//! branches.  Every resolver returns `None`.

use std::time::Duration;

pub const ENV_MASTER_SWITCH: &str = "GROK_EXTERNAL_OTEL";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OtlpTransport {
    #[default]
    HttpProtobuf,
    Grpc,
}

impl OtlpTransport {
    pub fn as_protocol_str(self) -> &'static str {
        match self {
            Self::HttpProtobuf => "http/protobuf",
            Self::Grpc => "grpc",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExporterSelection {
    #[default]
    None,
    Otlp,
    Console,
}

impl ExporterSelection {
    pub fn is_active(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ContentGates {
    pub log_user_prompts: bool,
    pub log_tool_details: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TemporalityPreference {
    #[default]
    Delta,
    Cumulative,
}

#[derive(Debug, Clone, Default)]
pub struct ExternalClientInfo {
    pub service_version: String,
    pub client_version: String,
    pub app_entrypoint: String,
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ExternalOtelFileConfig {
    pub enabled: Option<bool>,
    pub metrics_exporter: Option<String>,
    pub logs_exporter: Option<String>,
    pub endpoint: Option<String>,
    pub protocol: Option<String>,
    pub certificate: Option<String>,
    pub client_certificate: Option<String>,
    pub client_key: Option<String>,
    pub log_user_prompts: Option<bool>,
    pub log_tool_details: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ExternalOtelConfig {
    pub metrics_exporter: ExporterSelection,
    pub logs_exporter: ExporterSelection,
    pub logs_transport: OtlpTransport,
    pub metrics_transport: OtlpTransport,
    pub logs_endpoint: String,
    pub metrics_endpoint: String,
    pub logs_headers: Vec<(String, String)>,
    pub metrics_headers: Vec<(String, String)>,
    pub logs_ca_certificate: Option<String>,
    pub metrics_ca_certificate: Option<String>,
    pub logs_client_certificate: Option<String>,
    pub logs_client_key: Option<String>,
    pub metrics_client_certificate: Option<String>,
    pub metrics_client_key: Option<String>,
    pub timeout: Duration,
    pub metric_export_interval: Duration,
    pub logs_export_interval: Duration,
    pub gates: ContentGates,
    pub temporality: TemporalityPreference,
    pub include_session_id_on_metrics: bool,
    pub include_version_on_metrics: bool,
    pub client: ExternalClientInfo,
    pub internal_pipeline_consumed_otel_vars: bool,
    pub enabled_source: &'static str,
}

impl ExternalOtelConfig {
    pub fn resolve(_file: Option<&ExternalOtelFileConfig>) -> Option<Self> {
        None
    }

    pub fn resolve_with(
        _getenv: impl Fn(&str) -> Option<String>,
        _file: Option<&ExternalOtelFileConfig>,
    ) -> Option<Self> {
        None
    }
}

pub fn parse_header_list(_raw: &str) -> Vec<(String, String)> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_activation_path_is_removed() {
        assert!(ExternalOtelConfig::resolve(None).is_none());
        assert!(ExternalOtelConfig::resolve_with(|_| Some("1".into()), None).is_none());
        assert!(!ExporterSelection::Otlp.is_active());
        assert!(parse_header_list("authorization=secret").is_empty());
    }
}
