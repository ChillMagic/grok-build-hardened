// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! No-upload trace compatibility facade.
//!
//! The upstream artifact collectors, repository/session archive builders,
//! upload queue resolver, retry/recovery workers, and cloud writers are
//! deliberately deleted. Functions retained here never serialize or read a
//! payload for transmission.

use super::turn::{PromptTraceContext, UploadWait};
use crate::sampling::types::ToolDefinition;
use crate::session::repo_changes::TraceExportConfig;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::oneshot;
use xai_file_utils::queue::UploadQueue;
use xai_grok_workspace::permission::PermissionEvent;

pub(crate) use prod_mc_cli_chat_proxy_types::{
    GCS_SCHEMA_VERSION, LocalSandboxTelemetry, PromptMetadata, PromptMetadataParams,
};

pub(crate) async fn upload_tool_definitions(
    _gcs_config: TraceExportConfig,
    _auth_manager: Option<Arc<crate::auth::AuthManager>>,
    _tool_definitions: &[ToolDefinition],
    _artifact_tracker: Option<&super::manifest::ArtifactTracker>,
) {
}

pub(crate) async fn upload_session_state(
    _ctx: &PromptTraceContext,
    _phase: &str,
    _session_copy_rx: oneshot::Receiver<
        anyhow::Result<crate::session::persistence::SessionStateCopy>,
    >,
    _wait: UploadWait,
) -> super::turn::UploadOutcome {
    super::turn::UploadOutcome::Failed {
        reason: "upload_capability_removed",
        status_code: None,
    }
}

pub(crate) fn local_sandbox_telemetry() -> Option<LocalSandboxTelemetry> {
    None
}

pub(crate) fn strip_url_credentials(_url_str: &str) -> String {
    String::new()
}

pub(crate) async fn resolve_git_repo_info(_cwd: &str) -> (Option<String>, Option<String>) {
    (None, None)
}

pub(crate) async fn enrich_git_metadata(_ctx: &PromptTraceContext, _metadata: &mut PromptMetadata) {
}

pub(crate) async fn upload_metadata(_ctx: &PromptTraceContext, _metadata: PromptMetadata) {}

pub(crate) async fn upload_subagent_metadata(
    _metadata: &crate::agent::subagent::SubagentSessionMetadata,
    _bucket_url: &str,
    _upload_method: crate::session::repo_changes::UploadMethod,
    _auth_manager: std::sync::Arc<crate::auth::AuthManager>,
) {
}

pub(crate) async fn upload_images(
    _ctx: &PromptTraceContext,
    _images: &[agent_client_protocol::ImageContent],
) {
}

pub(crate) fn mime_type_to_extension(mime_type: &str) -> &str {
    match mime_type {
        "image/png" => "png",
        "image/jpeg" | "image/jpg" => "jpeg",
        "image/gif" => "gif",
        "image/webp" => "webp",
        "image/svg+xml" => "svg",
        "image/bmp" => "bmp",
        "image/tiff" => "tiff",
        "image/heic" => "heic",
        "image/heif" => "heif",
        "image/avif" => "avif",
        _ => "bin",
    }
}

pub(crate) async fn upload_full_prompt_txt(_ctx: &PromptTraceContext, _full_prompt: &str) {}

pub(crate) async fn upload_plugin_state(
    _ctx: &PromptTraceContext,
    _registry: Option<&xai_grok_agent::plugins::PluginRegistry>,
) {
}

pub(crate) async fn upload_artifact_to_gcs(
    _ctx: &PromptTraceContext,
    _gcs_path: &str,
    _content: &[u8],
    _content_type: &str,
    _artifact: &str,
) -> Option<String> {
    None
}

pub(crate) async fn upload_small_artifact(
    _ctx: &PromptTraceContext,
    _content: &[u8],
    _gcs_path: &str,
    _content_type: &str,
    _artifact_name: &str,
    _wait: UploadWait,
) {
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct SubagentSpawnedRef {
    pub(crate) subagent_id: String,
    pub(crate) child_session_id: String,
    pub(crate) subagent_type: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub(crate) description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) persona: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) resumed_from: Option<String>,
}

#[derive(serde::Serialize)]
pub(crate) struct TurnResultMetadata {
    pub(crate) schema_version: &'static str,
    pub(crate) request_id: String,
    pub(crate) completed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stop_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) total_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) input_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cached_input_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) output_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) error: Option<String>,
    pub(crate) finished_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) signals: Option<crate::session::signals::SessionSignals>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) turn_delta: Option<crate::session::signals::SessionSignalsDelta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) resolved_model: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) subagents_spawned: Vec<SubagentSpawnedRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) start_prompt_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) end_prompt_mode: Option<String>,
}

pub(crate) async fn upload_turn_result(
    _ctx: &PromptTraceContext,
    _result: &TurnResultMetadata,
    _wait: UploadWait,
) {
}

pub(crate) async fn upload_streaming_partial(
    _ctx: &PromptTraceContext,
    _capture: &crate::session::acp_session::StreamingTurnCapture,
    _wait: UploadWait,
) {
}

pub(crate) enum SessionMetadataType {
    Share,
}

pub(crate) async fn upload_session_metadata(
    _ctx: &PromptTraceContext,
    _metadata_type: SessionMetadataType,
) {
}

pub(crate) async fn upload_memory_state(_ctx: &PromptTraceContext) {}

pub(crate) async fn upload_unified_log(_ctx: &PromptTraceContext, _wait: UploadWait) {}

pub(crate) async fn upload_permission_events(
    _ctx: &PromptTraceContext,
    _events: &[PermissionEvent],
    _wait: UploadWait,
) {
}

pub(crate) async fn upload_turn_messages(
    _ctx: &PromptTraceContext,
    _capture: xai_chat_state::TurnCapture,
    _wait: UploadWait,
) -> bool {
    true
}

#[derive(Debug)]
pub(crate) struct SessionStateBuildError {
    pub reason: &'static str,
    pub error: anyhow::Error,
}

fn session_archive_removed() -> SessionStateBuildError {
    SessionStateBuildError {
        reason: "upload_capability_removed",
        error: anyhow::anyhow!(crate::privacy_build::REMOVED_MESSAGE),
    }
}

pub(crate) async fn build_chat_history_session_state(
    _messages: &[xai_grok_sampling_types::conversation::ConversationItem],
) -> Result<Vec<u8>, SessionStateBuildError> {
    Err(session_archive_removed())
}

pub(crate) async fn build_chat_history_then_move_capture(
    capture: xai_chat_state::TurnCapture,
) -> (
    Result<Vec<u8>, SessionStateBuildError>,
    xai_chat_state::TurnCapture,
) {
    (Err(session_archive_removed()), capture)
}

pub(crate) async fn upload_harness_session_archive(
    _ctx: &PromptTraceContext,
    _tar: Result<Vec<u8>, SessionStateBuildError>,
) -> bool {
    false
}

pub(crate) fn spawn_startup_spill_reconcile(
    _grok_home: std::path::PathBuf,
    _queue: Option<UploadQueue>,
) {
}

pub(crate) async fn flush_upload_queue(
    _ctx: &PromptTraceContext,
    _deadline: tokio::time::Instant,
) -> usize {
    0
}

pub(crate) fn blocking_attempt_budget(_deadline: tokio::time::Instant) -> std::time::Duration {
    std::time::Duration::ZERO
}

pub(crate) async fn flush_then_write_error_manifest(
    _ctx: &PromptTraceContext,
    _deadline: tokio::time::Instant,
) {
}

pub(crate) fn purge_stale_upload_scratch_dir(_scratch_dir: &Path) -> std::io::Result<bool> {
    Ok(false)
}

pub(crate) fn spawn_purge_stale_upload_scratch() {}

pub(crate) fn spawn_upload_queue(
    _grok_home: &Path,
    _gcs_config: &TraceExportConfig,
    client_version: Option<&str>,
    _auth_manager: Arc<crate::auth::AuthManager>,
) -> UploadQueue {
    let queue = UploadQueue::disabled();
    if let Some(version) = client_version {
        queue.with_client_version(version)
    } else {
        queue
    }
}

pub(crate) async fn upload_trace_artifact_deferred(
    _ctx: &PromptTraceContext,
    _content: &[u8],
    _gcs_path: &str,
    _content_type: &str,
    _artifact_name: &str,
    _deadline: tokio::time::Instant,
) -> anyhow::Result<()> {
    Err(anyhow::anyhow!(crate::privacy_build::REMOVED_MESSAGE))
}

pub(crate) async fn upload_trace_artifact(
    _ctx: &PromptTraceContext,
    _content: &[u8],
    _gcs_path: &str,
    _content_type: &str,
    _artifact_name: &str,
) {
}

#[cfg(test)]
mod tests {
    #[test]
    fn trace_sender_is_absent() {
        assert!(!crate::privacy_build::PASSIVE_UPLOADS_COMPILED_IN);
    }
}
