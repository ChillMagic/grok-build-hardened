// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Inert facade for removed server-delivered agents, skills, roles, personas,
//! and workflows.

use super::ExtResult;
use crate::agent::MvpAgent;
use agent_client_protocol as acp;
use serde::Serialize;
use std::path::Path;
use std::time::Duration;

pub(crate) const BUNDLE_SYNC_TTL: Duration = Duration::from_secs(60 * 60);
pub(crate) const NO_BUNDLE_CREDENTIALS_ERROR: &str =
    "server-delivered bundles are disabled by the privacy build";

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BundleSyncResult {
    pub updated: bool,
    pub version: String,
    pub personas_count: usize,
    pub roles_count: usize,
    pub agents_count: usize,
    pub skills_count: usize,
}

pub async fn handle(_agent: &MvpAgent, _args: &acp::ExtRequest) -> ExtResult {
    Err(acp::Error::internal_error().data(crate::privacy_build::REMOVED_MESSAGE))
}

pub(crate) fn has_bundle_credentials(
    _auth_manager: Option<&std::sync::Arc<crate::auth::AuthManager>>,
    _deployment_key: Option<&str>,
) -> bool {
    false
}

pub(crate) fn bundle_cache_is_fresh(_root: &Path, _ttl: Duration) -> bool {
    false
}

pub(crate) async fn maybe_sync_bundle_to_root(
    _root: &Path,
    _proxy_base_url: &str,
    _auth_manager: Option<&std::sync::Arc<crate::auth::AuthManager>>,
    _deployment_key: Option<&str>,
    _alpha_test_key: Option<&str>,
    _force: bool,
    _ttl: Duration,
) -> anyhow::Result<Option<BundleSyncResult>> {
    Ok(None)
}

pub(crate) async fn sync_bundle_to_root(
    _root: &Path,
    _proxy_base_url: &str,
    _auth_manager: Option<&std::sync::Arc<crate::auth::AuthManager>>,
    _deployment_key: Option<&str>,
    _alpha_test_key: Option<&str>,
    _force: bool,
) -> anyhow::Result<BundleSyncResult> {
    anyhow::bail!(NO_BUNDLE_CREDENTIALS_ERROR)
}
