//! Disabled rollout-survey telemetry endpoint.

use agent_client_protocol as acp;

use super::ExtResult;
use crate::agent::MvpAgent;

pub async fn handle(_agent: &MvpAgent, _args: &acp::ExtRequest) -> ExtResult {
    Err(acp::Error::internal_error().data(crate::privacy_build::REMOVED_MESSAGE))
}
