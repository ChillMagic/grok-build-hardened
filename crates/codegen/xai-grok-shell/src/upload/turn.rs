// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! No-upload turn compatibility facade.

use crate::session::repo_changes::TraceExportConfig;
use tokio::sync::oneshot;
use xai_grok_workspace::permission::PermissionEvent;

pub(crate) struct SyntheticTurnTraceRequest {
    pub session_id: agent_client_protocol::SessionId,
    pub prompt_id: String,
    pub completion_rx: oneshot::Receiver<crate::session::commands::PromptTurnResult>,
    pub before_session_copy_rx:
        oneshot::Receiver<anyhow::Result<crate::session::persistence::SessionStateCopy>>,
}

pub(crate) enum UploadOutcome {
    Confirmed,
    Deferred,
    Failed {
        reason: &'static str,
        status_code: Option<u16>,
    },
}

impl UploadOutcome {
    pub(crate) fn is_confirmed(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum UploadWait {
    Confirm,
    Defer { deadline: tokio::time::Instant },
}

pub(crate) use xai_grok_telemetry::session_metrics::TraceUploadReason;

#[derive(Clone)]
pub(crate) struct PromptTraceContext {
    pub(crate) gcs_config: TraceExportConfig,
    pub(crate) session_info: crate::session::info::Info,
    pub(crate) turn_number: u64,
    pub(crate) session_handle: crate::session::SessionHandle,
    pub(crate) session_registry_enabled: bool,
    pub(crate) upload_queue: Option<xai_file_utils::queue::UploadQueue>,
    pub(crate) artifact_tracker: super::manifest::ArtifactTracker,
    pub(crate) auth_manager: std::sync::Arc<crate::auth::AuthManager>,
}

impl PromptTraceContext {
    pub(crate) fn artifact_upload_context(&self) -> super::manifest::ArtifactUploadContext {
        super::manifest::ArtifactUploadContext {
            gcs_config: self.gcs_config.clone(),
            artifact_tracker: self.artifact_tracker.clone(),
        }
    }
}

/// Upload futures are dropped without being polled; this creates no task.
pub(crate) fn spawn_upload_task<F>(_task_name: &'static str, fut: F)
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    drop(fut);
}

#[cfg(test)]
pub(crate) async fn join_required_restore_artifacts<Fs, Fp, Fm>(
    session_state: Fs,
    permission_events: Fp,
    memory: Fm,
) -> UploadOutcome
where
    Fs: std::future::Future<Output = UploadOutcome>,
    Fp: std::future::Future<Output = ()>,
    Fm: std::future::Future<Output = ()>,
{
    let (outcome, _, _) = futures::join!(session_state, permission_events, memory);
    outcome
}

/// Preserve local capture-slot cleanup, but return data only to the caller;
/// no uploader consumes it in this build.
pub(crate) async fn take_streaming_partial(
    cmd_tx: &tokio::sync::mpsc::UnboundedSender<crate::session::SessionCommand>,
    prompt_id: String,
    committed: bool,
    model_id: Option<String>,
) -> Option<crate::session::acp_session::StreamingTurnCapture> {
    use crate::session::SessionCommand;
    let (tx, rx) = oneshot::channel();
    if cmd_tx
        .send(SessionCommand::TakeStreamingCapture {
            prompt_id,
            respond_to: tx,
        })
        .is_err()
    {
        return None;
    }
    let taken = rx.await.ok().flatten();
    if committed {
        return taken
            .filter(|capture| capture.has_doom_loop_segments())
            .map(|mut capture| {
                if capture.model_id.is_none() {
                    capture.model_id = model_id;
                }
                capture
                    .reason
                    .get_or_insert_with(|| "doom_loop_recovered".to_owned());
                capture
            });
    }
    taken.map(|mut capture| {
        if capture.model_id.is_none() {
            capture.model_id = model_id;
        }
        capture
    })
}

pub(crate) async fn complete_prompt_trace(
    _ctx: PromptTraceContext,
    _permission_events: Vec<PermissionEvent>,
    _session_copy_rx: oneshot::Receiver<
        anyhow::Result<crate::session::persistence::SessionStateCopy>,
    >,
    _turn_messages: Option<xai_chat_state::TurnCapture>,
    _streaming_partial: Option<crate::session::acp_session::StreamingTurnCapture>,
    _wait: UploadWait,
) -> anyhow::Result<bool> {
    Ok(false)
}

pub(crate) fn parse_agent_profile_from_meta(
    meta: Option<&agent_client_protocol::Meta>,
) -> Option<xai_grok_agent::AgentDefinition> {
    let value = meta?.get("agentProfile")?;
    if value.is_object() {
        return xai_grok_agent::AgentDefinition::from_json(value).ok();
    }
    value.as_str().and_then(xai_grok_agent::discovery::by_name)
}

pub(crate) fn parse_ask_user_question_from_meta(
    meta: Option<&agent_client_protocol::Meta>,
) -> Option<bool> {
    meta?.get("askUserQuestion")?.as_bool()
}

pub(crate) fn lookup_session_model(
    session_model: Option<agent_client_protocol::ModelId>,
    default_model_id: &agent_client_protocol::ModelId,
) -> agent_client_protocol::ModelId {
    session_model.unwrap_or_else(|| default_model_id.clone())
}

pub(crate) fn apply_yolo_mode_to_matching_sessions<'a>(
    sessions: impl IntoIterator<Item = &'a mut crate::session::SessionHandle>,
    sender_id: Option<&str>,
    yolo_mode: bool,
) -> usize {
    let matches_sender = |handle: &crate::session::SessionHandle| -> bool {
        sender_id.is_none()
            || handle
                .origin_client
                .as_ref()
                .map(|client| client.product.as_str())
                == sender_id
    };
    let mut updated = 0;
    for handle in sessions {
        if matches_sender(handle) {
            handle.yolo_mode = yolo_mode;
            let _ = handle
                .cmd_tx
                .send(crate::session::SessionCommand::SetYoloMode { enabled: yolo_mode });
            updated += 1;
        }
    }
    updated
}

#[cfg(test)]
mod tests {
    #[test]
    fn no_upload_can_be_confirmed() {
        assert!(!super::UploadOutcome::Confirmed.is_confirmed());
    }
}
