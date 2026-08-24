//! Fail-closed HTTP hook runner for the privacy build.
//!
//! The upstream implementation serializes hook event envelopes and POSTs them
//! to configurable URLs. That is a passive data-egress capability, so the
//! implementation and its HTTP client are removed from this build.

use std::time::Instant;

use crate::config::HookSpec;
use crate::event::HookEventEnvelope;

use super::{GateKind, HookRunOutput, HookRunnerResult, RunContext};

/// Reject every HTTP hook without DNS resolution, socket creation, or payload
/// serialization. Local command hooks remain available; HTTP hooks cannot be
/// re-enabled by project files, plugins, environment variables, or responses.
pub async fn run_http_hook(
    _spec: &HookSpec,
    _envelope: &HookEventEnvelope,
    _ctx: &RunContext<'_>,
    _mode: GateKind,
) -> HookRunOutput {
    let start = Instant::now();
    (
        HookRunnerResult::Failed("HTTP hooks were removed from this privacy build".to_string()),
        start.elapsed(),
        None,
    )
}
