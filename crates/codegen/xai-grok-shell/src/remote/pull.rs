//! Removed pull-on-miss cloud session hydration.

use crate::remote::client::{BackendClient, BackendError};

#[derive(Debug)]
pub enum PullResult {
    Hydrated(crate::session::info::Info),
    NotFound,
}

pub async fn pull_session_to_local(
    _session_id: &str,
    _client: &BackendClient,
) -> Result<PullResult, BackendError> {
    Ok(PullResult::NotFound)
}
