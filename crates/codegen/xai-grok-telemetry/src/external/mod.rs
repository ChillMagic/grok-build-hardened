//! No-network external OTEL facade for the privacy build.
//!
//! Exporter construction, HTTP/gRPC providers, redaction pipelines, and
//! background workers are intentionally deleted.

pub mod config;
pub mod schema;

pub use config::{ContentGates, ExternalOtelConfig, ExternalOtelFileConfig};

#[derive(Debug, Clone, Default)]
pub struct IdentityAttrs {
    pub user_id: Option<String>,
    pub organization_id: Option<String>,
    pub team_id: Option<String>,
    pub deployment_id: Option<String>,
}

impl IdentityAttrs {
    pub fn from_snapshot(snapshot: &xai_grok_auth::CredentialSnapshot) -> Self {
        Self {
            user_id: snapshot.user_id.clone(),
            organization_id: snapshot.organization_id.clone(),
            team_id: snapshot.team_id.clone(),
            deployment_id: snapshot.deployment_id.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ExternalOtelRemotePolicy {
    pub force_disable: bool,
    pub lock_content_gates: bool,
}

pub struct ExternalTelemetry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ExportHealthSnapshot {
    pub records_dropped: u64,
    pub metric_exports_dropped: u64,
    pub export_failures: u64,
    pub export_successes: u64,
}

pub fn init(_cfg: Option<ExternalOtelConfig>) {}

pub fn set_settings_gate_max_wait(_max_wait: std::time::Duration) {}

pub fn settings_gate_max_wait() -> std::time::Duration {
    std::time::Duration::ZERO
}

pub fn suppress_external_otel_until_settings() {}

pub fn mark_external_otel_settings_resolved() {}

pub fn is_settings_gate_open() -> bool {
    false
}

pub fn is_active() -> bool {
    false
}

pub fn emit<T: crate::events::TelemetryEvent>(_data: &T) {}

pub fn set_identity(_attrs: IdentityAttrs) {}

pub fn apply_remote_policy(_policy: ExternalOtelRemotePolicy) {}

pub fn flush() {}

pub fn shutdown() {}

pub fn export_health() -> Option<ExportHealthSnapshot> {
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn external_export_is_permanently_inert() {
        assert!(!super::is_active());
        assert!(!super::is_settings_gate_open());
        assert!(super::export_health().is_none());
    }
}
