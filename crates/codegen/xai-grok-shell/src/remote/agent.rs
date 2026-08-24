//! Removed cloud-sandbox client.

use std::sync::Arc;

use crate::auth::AuthManager;
use anyhow::{Result, bail};

pub use prod_mc_cli_chat_proxy_types::{
    SandboxCreateEnvironmentRequest, SandboxEnvironment, SandboxEnvironmentResponse,
    SandboxEnvironmentVariable, SandboxEnvironmentWithMetadata, SandboxForkRequest,
    SandboxForkResponse, SandboxForkedSession, SandboxHibernateResponse,
    SandboxListEnvironmentsRequest, SandboxListEnvironmentsResponse,
    SandboxListPreinstalledPackagesResponse, SandboxLogsExitCodes, SandboxLogsResponse,
    SandboxMode, SandboxPreinstalledPackage, SandboxRestoreRequest, SandboxRestoreResponse,
    SandboxSecretInput, SandboxStartRequest, SandboxStartResponse, SandboxStatusResponse,
    SandboxTerminateRequest, SandboxUpdateEnvironmentRequest,
};

pub const CLOUD_SANDBOX_COMPILED_IN: bool = false;

pub struct SandboxClient {
    base_url: String,
    _auth_manager: Arc<AuthManager>,
}

impl SandboxClient {
    pub fn new(base_url: impl Into<String>, auth_manager: Arc<AuthManager>) -> Self {
        Self {
            base_url: base_url.into(),
            _auth_manager: auth_manager,
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn fork_session(&self, _request: &SandboxForkRequest) -> Result<SandboxForkResponse> {
        bail!(crate::privacy_build::REMOVED_MESSAGE)
    }

    pub(crate) async fn terminate_session(
        &self,
        _session_id: &str,
        _request: &SandboxTerminateRequest,
    ) -> Result<()> {
        bail!(crate::privacy_build::REMOVED_MESSAGE)
    }

    pub async fn list_environments(
        &self,
        _request: &SandboxListEnvironmentsRequest,
    ) -> Result<SandboxListEnvironmentsResponse> {
        bail!(crate::privacy_build::REMOVED_MESSAGE)
    }

    pub(crate) async fn create_environment(
        &self,
        _request: &SandboxCreateEnvironmentRequest,
    ) -> Result<SandboxEnvironmentResponse> {
        bail!(crate::privacy_build::REMOVED_MESSAGE)
    }

    pub(crate) async fn update_environment(
        &self,
        _environment_id: &str,
        _request: &SandboxUpdateEnvironmentRequest,
    ) -> Result<SandboxEnvironmentResponse> {
        bail!(crate::privacy_build::REMOVED_MESSAGE)
    }

    pub(crate) async fn delete_environment(&self, _environment_id: &str) -> Result<()> {
        bail!(crate::privacy_build::REMOVED_MESSAGE)
    }
}
