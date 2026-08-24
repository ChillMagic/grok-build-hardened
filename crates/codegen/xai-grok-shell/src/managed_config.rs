//! No-network managed-configuration facade for the privacy build.
//!
//! The upstream fetcher, response parser, signature-policy machinery,
//! background synchronizer, and artifact writer are deliberately deleted.
//! Neither local files nor server responses can re-enable this subsystem.

/// Compile-time marker consumed by the source audit.
pub const MANAGED_CONFIG_COMPILED_IN: bool = false;

/// Stable error returned by explicit managed-config operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManagedConfigError;

impl std::fmt::Display for ManagedConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("managed configuration was removed from this privacy build")
    }
}

impl std::error::Error for ManagedConfigError {}

/// Legacy cleanup hook retained as a no-op. The privacy loader ignores all
/// managed artifacts, so this function never mutates a user's files.
pub fn clear_orphan() {}

/// No background task is created.
pub fn spawn_sync(_cancel: tokio_util::sync::CancellationToken) {}

/// Keep deployment-key authentication usable without consulting a server or a
/// remotely written cache. The ID is derived locally from the caller's key.
pub fn resolve_deployment_id(deployment_key: Option<&str>) -> Option<String> {
    deployment_key
        .filter(|key| !key.is_empty())
        .map(crate::agent::config::deployment_id_from_key)
}

/// Resolve a deployment key only from the user's environment/config layer.
pub fn resolve_deployment_key() -> Option<String> {
    let config_val = crate::config::load_effective_config()
        .ok()
        .and_then(|root| {
            root.get("endpoints")?
                .get("deployment_key")?
                .as_str()
                .map(str::to_owned)
        });
    crate::agent::config::resolve_string_flag(
        None,
        "GROK_DEPLOYMENT_KEY",
        config_val.as_deref(),
        None,
    )
    .map(|resolved| resolved.value)
}

/// Remote managed-configuration fetch is a compile-time impossibility.
pub fn is_fetch_enabled() -> bool {
    false
}

/// No authenticated team principal is used for policy synchronization.
pub(crate) fn has_active_team_auth() -> bool {
    false
}

/// Compatibility API: no request and no write are performed.
pub async fn sync() -> Result<bool, ManagedConfigError> {
    Ok(false)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagedConfigSync {
    Skipped,
    Updated { is_team: bool },
    NoChange,
    Failed,
}

/// Login never triggers a policy fetch.
pub async fn post_login_sync(_authenticated: Option<crate::auth::GrokAuth>) -> ManagedConfigSync {
    ManagedConfigSync::Skipped
}

/// `grok setup` has no eligible managed-policy principal in this build.
pub fn has_principal() -> bool {
    false
}

/// Managed policy is never a serving configuration layer.
pub fn current_serving_identity() -> crate::config::ServingIdentity {
    crate::config::ServingIdentity::None
}

pub fn active_team_id_any_expiry() -> Option<String> {
    None
}

/// Local startup labeling only; it cannot affect configuration.
pub fn classify_auth_mode() -> xai_grok_telemetry::startup::AuthMode {
    if resolve_deployment_key().is_some() {
        xai_grok_telemetry::startup::AuthMode::Deployment
    } else {
        xai_grok_telemetry::startup::AuthMode::Personal
    }
}

/// Session startup performs no remote policy operation.
pub async fn ensure_managed_policy_present(
    _auth_manager: &std::sync::Arc<crate::auth::AuthManager>,
) {
}

/// There is no cloud-managed policy gate in this fork.
pub fn managed_policy_gate() -> Result<(), String> {
    Ok(())
}

#[derive(Debug)]
pub enum SetupOutcome {
    Installed,
    NothingConfigured,
    Skipped,
    Failed(ManagedConfigError),
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetupReport {
    pub source: Option<&'static str>,
    pub configured: bool,
    pub deployment_id: Option<String>,
    pub team_id: Option<String>,
    pub managed_config: Option<String>,
    pub requirements: Option<String>,
    pub fail_closed: bool,
}

pub async fn fetch_setup_report() -> Result<SetupReport, ManagedConfigError> {
    Err(ManagedConfigError)
}

pub async fn run_setup() -> SetupOutcome {
    SetupOutcome::Skipped
}

#[cfg(test)]
mod tests {
    #[test]
    fn managed_configuration_is_permanently_inert() {
        assert!(!super::MANAGED_CONFIG_COMPILED_IN);
        assert!(!super::is_fetch_enabled());
        assert!(!super::has_principal());
        assert_eq!(
            super::current_serving_identity(),
            crate::config::ServingIdentity::None
        );
    }
}
