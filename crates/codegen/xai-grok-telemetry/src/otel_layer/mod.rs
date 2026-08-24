// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! No-export tracing layer for the privacy build.

use std::sync::Arc;

use tracing_subscriber::registry::LookupSpan;
use xai_grok_auth::AuthCredentialProvider;

pub struct OtelLayerConfig {
    pub credentials: Arc<dyn AuthCredentialProvider>,
    pub token_header_value: String,
    pub alpha_test_key: Option<String>,
    pub exporter: OtelExporterConfig,
}

#[derive(Debug, Clone, Copy)]
pub struct OtelClientInfo {
    pub client_name: &'static str,
    pub client_version: &'static str,
    pub service_version: &'static str,
    pub app_entrypoint: &'static str,
}

#[derive(Debug, Default, Clone)]
pub struct OtelExporterConfig {
    pub traces_url: String,
    pub extra_headers: Vec<(String, String)>,
    pub export_interval: Option<std::time::Duration>,
    pub timeout: Option<std::time::Duration>,
    pub enabled: bool,
}

pub fn build_otel_layer<S>(
    _client: OtelClientInfo,
    _config: OtelLayerConfig,
) -> impl tracing_subscriber::layer::Layer<S>
where
    S: tracing::Subscriber + for<'span> LookupSpan<'span>,
{
    tracing_subscriber::layer::Identity::new()
}

pub fn shutdown_otel() {
    crate::external::shutdown();
}

pub struct OtelGuard;

impl Drop for OtelGuard {
    fn drop(&mut self) {
        shutdown_otel();
    }
}

pub fn otel_guard() -> OtelGuard {
    OtelGuard
}

#[cfg(test)]
mod tests {
    #[test]
    fn exporter_marker_is_off() {
        assert!(!crate::NETWORK_TELEMETRY_COMPILED_IN);
    }
}
