//! No-network feedback/signals compatibility facade.
//!
//! Wire data types remain for local state compatibility. Every transport
//! method returns an error before serializing or reading its payload.

use anyhow::Result;

// Import feedback wire types from cli-chat-proxy
use prod_mc_cli_chat_proxy_types::feedback_types::{
    ClientType, CreateFeedbackRequestInput, CreateFeedbackRequestResponse,
    FeedbackHeuristicsConfig, FeedbackRequestUpdateResponse, FeedbackResponse, FeedbackSubmission,
    SessionEventRequest, SessionEventResponse, SessionSignalsUpdate, SessionSignalsUpdateResponse,
};

// ============================================================================
// Turn delta wire types (local to xai-grok-shell until cli-chat-proxy catches up)
// ============================================================================

/// Per-turn delta sent at the end of every turn via
/// `POST /v1/sessions/{session_id}/turn-deltas`.
///
/// Each field falls into one of four categories:
///
/// - **Delta** — the *change* since the previous turn end (computed as
///   `current_cumulative - previous_turn_snapshot`). For the first turn,
///   the previous snapshot is zero.
/// - **Turn-level** — an absolute value measured only for *this* turn,
///   reset between turns. `None` when the event did not occur this turn.
/// - **Accumulated** — a cumulative total since session start, monotonically
///   increasing across turns.
/// - **Context** — session/turn metadata that is neither a counter nor a
///   measurement (e.g. IDs, timestamps, client type).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SessionTurnDelta {
    // ── Context fields ──────────────────────────────────────────────────
    /// **[context]** Which client surface produced this record (e.g. CLI, TUI).
    pub client_type: ClientType,

    /// **[context]** 1-based turn number at the time of this snapshot. Equals
    /// the cumulative `turn_count` from `SessionSignals`.
    pub turn_number: i64,

    // ── Delta counters ──────────────────────────────────────────────────
    // Each is `current_cumulative - previous_turn_snapshot`.
    /// **[delta]** Number of tool calls made during this turn.
    pub delta_tool_calls: i64,

    /// **[delta]** Number of tool calls that failed during this turn.
    pub delta_tool_failures: i64,

    /// **[delta]** Number of errors (including sampling errors) during this turn.
    pub delta_errors: i64,

    /// **[delta]** Number of user cancellations (Ctrl+C) during this turn.
    pub delta_cancellations: i64,

    /// **[delta]** Number of regeneration requests during this turn.
    pub delta_regenerations: i64,

    /// **[delta]** Number of conversation compactions during this turn.
    pub delta_compactions: i64,

    /// **[delta]** Number of edit-and-retry actions (user rewinds prompt)
    /// during this turn.
    pub delta_edit_and_retries: i64,

    /// **[delta]** Number of positive ratings (thumbs-up) during this turn.
    pub delta_positive_ratings: i64,

    /// **[delta]** Number of negative ratings (thumbs-down) during this turn.
    pub delta_negative_ratings: i64,

    /// **[delta]** Number of assistant messages produced during this turn
    /// (may be >1 when tool-call rounds generate intermediate messages).
    pub delta_assistant_messages: i64,

    /// **[delta]** Number of long idle pauses (>60 s) that occurred during
    /// this turn.
    pub delta_long_pauses: i64,

    /// **[delta]** Number of successful tool uses during this turn. Derived
    /// as `delta_tool_calls − delta_tool_failures`.
    pub delta_successful_tool_uses: i64,

    // ── Turn-level snapshot values ──────────────────────────────────────
    /// **[turn-level]** Consecutive cancellation streak at turn end. This is
    /// a point-in-time snapshot (not a diff) — it resets to 0 when a turn
    /// completes normally.
    pub consecutive_cancellations: i64,

    // ── Turn-level latency ──────────────────────────────────────────────
    // Absolute measurements for this turn's inference request only.
    // `None` when no inference occurred during the turn.
    /// **[turn-level]** Time-to-first-token for this turn's model response
    /// (milliseconds). `None` when no inference occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_first_token_ms: Option<i64>,

    /// **[turn-level]** Total wall-clock response time for this turn's model
    /// response (milliseconds). `None` when no inference occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_response_time_ms: Option<i64>,

    /// **[turn-level]** Inter-token latency p50 for this turn (ms).
    /// Computed from the token intervals collected during this turn only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub itl_p50_ms: Option<i64>,

    /// **[turn-level]** Inter-token latency p99 for this turn (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub itl_p99_ms: Option<i64>,

    /// **[turn-level]** Inter-token latency maximum for this turn (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub itl_max_ms: Option<i64>,

    /// **[turn-level]** Inter-token latency mean for this turn (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub itl_mean_ms: Option<i64>,

    // ── Accumulated / snapshot session-level values ─────────────────────
    /// **[accumulated]** Current context window usage as a percentage (0–100)
    /// at turn end. Read from cumulative `SessionSignals.context_window_usage`.
    pub context_window_usage: i64,

    /// **[accumulated]** Primary model ID (most recently used model). Read
    /// from cumulative `SessionSignals.primary_model_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,

    // ── Turn-level outcome / served checkpoint ──────────────────────────
    /// Whole-turn wall-clock duration (prompt→final response), ms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_duration_ms: Option<i64>,

    /// Terminal outcome: `"completed"` | `"cancelled"` | `"error"`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_outcome: Option<String>,

    /// Served model fingerprint (upstream `system_fingerprint`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_fingerprint: Option<String>,

    // ── Turn-level tool / error detail ──────────────────────────────────
    /// **[turn-level]** Distinct tool names invoked during this turn
    /// (deduplicated, sorted, capped at 100 entries). Reset each turn.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools_used_this_turn: Vec<String>,

    /// **[turn-level]** Error type strings that occurred during this turn
    /// (e.g. `"timeout"`, `"rate_limit"`, `"tool_error"`). Reset each turn.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub error_types_this_turn: Vec<String>,

    /// **[turn-level]** Per-tool success/failure breakdown for this turn,
    /// JSON-serialized array of `{ tool_name, successes, failures }`.
    /// Empty string when no tool calls occurred. Reset each turn.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub tool_outcomes: String,

    // ── Accumulated totals ──────────────────────────────────────────────
    /// **[accumulated]** Total tool calls since session start.
    /// Read from cumulative `SessionSignals.tool_call_count`.
    pub cumulative_tool_calls: i64,

    /// **[accumulated]** Total errors since session start.
    /// Read from cumulative `SessionSignals.error_count`.
    pub cumulative_errors: i64,

    /// **[accumulated]** Wall-clock seconds elapsed since session start.
    /// Read from cumulative `SessionSignals.session_duration_seconds`.
    pub session_duration_seconds: i64,

    /// **[accumulated]** Sum of token counts across all compactions since
    /// session start. Read from `SessionSignals.total_tokens_before_compaction`.
    #[serde(default)]
    pub total_tokens_before_compaction: i64,

    /// **[context]** Arbitrary JSON metadata blob.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

    /// **[context]** Prompt/request ID that initiated this turn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,

    /// **[context]** Wall-clock time when the session was created. Used for
    /// BQ partitioning on the backend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_start_at: Option<chrono::DateTime<chrono::Utc>>,

    // ── Feedback state ──────────────────────────────────────────────────
    /// **[accumulated]** Total number of feedback requests sent this session.
    /// Supplied by `FeedbackHeuristics`, not the signals actor.
    #[serde(default)]
    pub feedback_requests_sent: i64,

    /// **[accumulated]** Wall-clock timestamp of the most recent feedback
    /// request sent this session. Supplied by `FeedbackHeuristics`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_feedback_request_at: Option<chrono::DateTime<chrono::Utc>>,

    // ── Turn-level token counts ─────────────────────────────────────────
    /// **[turn-level]** Number of response (completion minus reasoning)
    /// tokens generated during this turn. `None` when no inference occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_tokens: Option<i64>,

    /// **[turn-level]** Number of thinking/reasoning tokens generated during
    /// this turn. `None` when no inference occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thinking_tokens: Option<i64>,

    // ── LOC Attribution Deltas ──────────────────────────────────────────
    // Each is `current_cumulative - previous_turn_snapshot`, same as the
    // counter deltas above. Tracks lines-of-code changes attributed to
    // the agent vs. the human during this turn.
    /// **[delta]** Lines added by the agent during this turn.
    #[serde(default)]
    pub delta_agent_lines_added: i64,

    /// **[delta]** Lines removed by the agent during this turn.
    #[serde(default)]
    pub delta_agent_lines_removed: i64,

    /// **[delta]** Agent-added lines that were reverted during this turn.
    #[serde(default)]
    pub delta_agent_lines_added_reverted: i64,

    /// **[delta]** Agent-removed lines that were reverted during this turn.
    #[serde(default)]
    pub delta_agent_lines_removed_reverted: i64,

    /// **[delta]** Lines added by the human during this turn.
    #[serde(default)]
    pub delta_human_lines_added: i64,

    /// **[delta]** Lines removed by the human during this turn.
    #[serde(default)]
    pub delta_human_lines_removed: i64,

    /// **[delta]** Human-added lines that were reverted during this turn.
    #[serde(default)]
    pub delta_human_lines_added_reverted: i64,

    /// **[delta]** Human-removed lines that were reverted during this turn.
    #[serde(default)]
    pub delta_human_lines_removed_reverted: i64,

    /// **[delta]** New distinct files touched by the agent during this turn.
    #[serde(default)]
    pub delta_agent_files_touched: i64,

    /// **[delta]** New distinct files touched by the human during this turn.
    #[serde(default)]
    pub delta_human_files_touched: i64,

    /// **[delta]** New distinct files touched (union of agent + human)
    /// during this turn.
    #[serde(default)]
    pub delta_total_files_touched: i64,

    /// **[context]** Whether LOC (lines-of-code) attribution tracking was
    /// enabled for this session.  When `false`, all `delta_*` LOC fields
    /// above are meaningless zeros — the hunk tracker was never spawned.
    /// When `true`, zeros mean "tracking was active but no code changed."
    /// Defaults to `false` for backwards-compat with old clients that
    /// don't send this field.
    #[serde(default)]
    pub loc_tracking_enabled: bool,
}

/// Response from the turn-deltas endpoint.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SessionTurnDeltaResponse {
    pub session_id: String,
    pub turn_number: i64,
    pub recorded_at: chrono::DateTime<chrono::Utc>,
}

/// HTTP error from the feedback/signals API with a preserved status code.
///
/// Used to let callers distinguish auth failures (401) from transient errors
/// without fragile string matching on error messages.
#[derive(Debug, thiserror::Error)]
#[error("{context} failed with status {status}: {body}")]
pub(crate) struct FeedbackApiError {
    pub status: u16,
    pub context: &'static str,
    pub body: String,
}

impl FeedbackApiError {
    /// Returns `true` if this is a 401 Unauthorized response.
    pub(crate) fn is_unauthorized(&self) -> bool {
        self.status == 401
    }

    /// Returns `true` if this is a 403 Forbidden response.
    pub(crate) fn is_forbidden(&self) -> bool {
        self.status == 403
    }
}

/// Compile-compatible client facade. Network feedback collection is removed.
#[derive(Clone, Default)]
pub struct FeedbackClient {
    session_id: Option<String>,
}

impl FeedbackClient {
    pub fn new(_base_url: impl Into<String>, _user_token: Option<String>) -> Self {
        Self::default()
    }

    pub fn with_session_id(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    pub fn with_alpha_test_key(self, _key: Option<String>) -> Self {
        self
    }

    pub fn with_deployment_key(self, _key: Option<String>) -> Self {
        self
    }

    pub fn with_client<T>(
        _http: T,
        _base_url: impl Into<String>,
        _user_token: Option<String>,
    ) -> Self {
        Self::default()
    }

    pub(crate) fn with_auth_manager(
        self,
        _auth_manager: std::sync::Arc<crate::auth::AuthManager>,
    ) -> Self {
        self
    }

    pub(crate) fn has_token_refresher(&self) -> bool {
        false
    }

    pub(crate) async fn try_refresh_credentials(&self) -> bool {
        false
    }

    pub(crate) async fn wait_for_token_refresh(&self, _timeout: std::time::Duration) -> bool {
        false
    }

    pub(crate) fn is_auth_permanently_failed(&self) -> bool {
        true
    }

    fn removed<T>(&self) -> Result<T> {
        Err(anyhow::anyhow!(crate::privacy_build::REMOVED_MESSAGE))
    }

    pub async fn update_signals(
        &self,
        _session_id: &str,
        _update: &SessionSignalsUpdate,
    ) -> Result<SessionSignalsUpdateResponse> {
        self.removed()
    }

    pub async fn record_event(
        &self,
        _session_id: &str,
        _event: &SessionEventRequest,
    ) -> Result<SessionEventResponse> {
        self.removed()
    }

    pub async fn submit_feedback(
        &self,
        _submission: &FeedbackSubmission,
    ) -> Result<FeedbackResponse> {
        self.removed()
    }

    pub async fn complete_request(
        &self,
        _request_id: &str,
        _submission: &FeedbackSubmission,
    ) -> Result<()> {
        self.removed()
    }

    pub async fn dismiss_request(
        &self,
        _request_id: &str,
    ) -> Result<FeedbackRequestUpdateResponse> {
        self.removed()
    }

    pub async fn create_feedback_request(
        &self,
        _input: &CreateFeedbackRequestInput,
    ) -> Result<CreateFeedbackRequestResponse> {
        self.removed()
    }

    pub async fn get_feedback_config(&self) -> Result<FeedbackHeuristicsConfig> {
        self.removed()
    }

    pub(crate) async fn send_turn_delta(
        &self,
        _session_id: &str,
        _delta: &SessionTurnDelta,
    ) -> Result<SessionTurnDeltaResponse> {
        self.removed()
    }
}

/// Helper to create a SessionSignalsUpdate from local session signals.
pub fn signals_to_update(
    signals: &crate::session::signals::SessionSignals,
    client_type: ClientType,
) -> SessionSignalsUpdate {
    SessionSignalsUpdate {
        client_type,
        total_turns: Some(signals.turn_count as i64),
        user_message_count: Some(signals.user_message_count as i64),
        assistant_message_count: Some(signals.assistant_message_count as i64),
        cancellation_count: Some(signals.cancellation_count as i64),
        consecutive_cancellations: Some(signals.consecutive_cancellations as i64),
        error_count: Some(signals.error_count as i64),
        tool_failure_count: Some(signals.tool_failure_count as i64),
        tool_call_count: Some(signals.tool_call_count as i64),
        compaction_count: Some(signals.compaction_count as i64),
        regeneration_count: Some(signals.regeneration_count as i64),
        edit_and_retry_count: Some(signals.edit_and_retry_count as i64),
        positive_ratings: Some(signals.positive_ratings as i64),
        negative_ratings: Some(signals.negative_ratings as i64),
        long_pauses_count: Some(signals.long_pauses_count as i64),
        session_duration_seconds: Some(signals.session_duration_seconds as i64),
        tools_used: signals.tools_used.clone(),
        models_used: signals.models_used.clone(),
        primary_model_id: signals.primary_model_id.clone(),
        // Latency metrics
        avg_time_to_first_token_ms: Some(signals.avg_time_to_first_token_ms as i64),
        avg_response_time_ms: Some(signals.avg_response_time_ms as i64),
        min_time_to_first_token_ms: Some(signals.min_time_to_first_token_ms as i64),
        max_time_to_first_token_ms: Some(signals.max_time_to_first_token_ms as i64),
        latency_sample_count: Some(signals.latency_sample_count as i64),
        // ITL metrics (session-level aggregates)
        // Guard p50/p99 with itl_sample_count > 0 so that fresh sessions
        // (no ITL measured) send None → SQL NULL, preserving the "not yet
        // reported" semantic in the nullable PG columns.
        last_itl_p50_ms: signals.itl_p50_ms.map(|v| v as i64),
        last_itl_p99_ms: signals.itl_p99_ms.map(|v| v as i64),
        worst_itl_max_ms: signals.itl_max_ms.map(|v| v as i64),
        avg_itl_mean_ms: signals.itl_mean_ms.map(|v| v as i64),
        total_chunk_count: Some(signals.total_chunk_count as i64),
        itl_sample_count: Some(signals.itl_sample_count as i64),
        // Inference idle timeout tracing
        inference_idle_timeouts: Some(signals.inference_idle_timeouts as i64),
        inference_idle_timeout_configured_secs: signals
            .inference_idle_timeout_configured_secs
            .map(|v| v as i64),
        // Legacy client-side doom-loop detection removed; keep its columns null.
        doom_loop_warnings: None,
        doom_loop_terminations: None,
        doom_loop_threshold: None,
        doom_loop_ro_threshold: None,
        // Doom-loop recovery (server-detected, client-resampled) tracing
        doom_loop_recovery_fired: Some(
            signals.doom_loop_recovery_attempts > 0
                || signals.doom_loop_recovery_accepted_after_budget > 0,
        ),
        doom_loop_recovery_attempts: Some(signals.doom_loop_recovery_attempts as i64),
        doom_loop_recovery_accepted_after_budget: Some(
            signals.doom_loop_recovery_accepted_after_budget as i64,
        ),
        doom_loop_recovery_top_trigger: signals.doom_loop_recovery_top_trigger.clone(),
        doom_loop_recovery_aborted_chunks: Some(signals.doom_loop_recovery_aborted_chunks as i64),
        // GCS upload queue tracing
        gcs_queue_enqueued: Some(signals.gcs_queue_enqueued as i64),
        gcs_queue_uploaded: Some(signals.gcs_queue_uploaded as i64),
        gcs_queue_failed: Some(signals.gcs_queue_failed as i64),
        gcs_queue_fallbacks: Some(signals.gcs_queue_fallbacks as i64),
        gcs_queue_circuit_breaker_trips: Some(signals.gcs_queue_circuit_breaker_trips as i64),
        gcs_queue_pending: Some(signals.gcs_queue_pending as i64),
        gcs_queue_pending_bytes: Some(signals.gcs_queue_pending_bytes as i64),
        gcs_queue_orphans_cleaned: Some(signals.gcs_queue_orphans_cleaned as i64),
        // LOC Attribution
        agent_lines_added: Some(signals.agent_lines_added),
        agent_lines_removed: Some(signals.agent_lines_removed),
        agent_lines_added_reverted: Some(signals.agent_lines_added_reverted),
        agent_lines_removed_reverted: Some(signals.agent_lines_removed_reverted),
        human_lines_added: Some(signals.human_lines_added),
        human_lines_removed: Some(signals.human_lines_removed),
        human_lines_added_reverted: Some(signals.human_lines_added_reverted),
        human_lines_removed_reverted: Some(signals.human_lines_removed_reverted),
        agent_files_touched: Some(signals.agent_files_touched as i64),
        human_files_touched: Some(signals.human_files_touched as i64),
        total_files_touched: Some(signals.total_files_touched as i64),
        metadata: None,
    }
}

/// Build a `SessionTurnDelta` from a `TurnDeltaSnapshot` produced by the signals actor.
///
/// `feedback_requests_sent` and `last_feedback_request_at` are supplied by the
/// caller (from `FeedbackHeuristics`) because the signals actor does not track
/// feedback state.
/// `request_id` is the prompt/request identifier for this turn.
/// `loc_tracking_enabled` indicates whether the LOC attribution hunk tracker
/// was active for this session. When `false`, LOC delta fields are zeros
/// because the tracker was never spawned — not because no code changed.
pub(crate) fn snapshot_to_turn_delta(
    snapshot: &crate::session::signals::TurnDeltaSnapshot,
    client_type: ClientType,
    request_id: Option<String>,
    feedback_requests_sent: u32,
    last_feedback_request_at: Option<chrono::DateTime<chrono::Utc>>,
    loc_tracking_enabled: bool,
    turn_duration_ms: Option<i64>,
    turn_outcome: Option<String>,
    model_fingerprint: Option<String>,
) -> SessionTurnDelta {
    let metadata = {
        let mut metadata = serde_json::Map::new();
        if let Some(mode) = snapshot.start_prompt_mode.as_ref() {
            metadata.insert("startPromptMode".to_owned(), serde_json::json!(mode));
        }
        if let Some(mode) = snapshot.end_prompt_mode.as_ref() {
            metadata.insert("endPromptMode".to_owned(), serde_json::json!(mode));
        }
        (!metadata.is_empty()).then_some(serde_json::Value::Object(metadata))
    };
    let d = &snapshot.delta;
    let c = &snapshot.current;
    SessionTurnDelta {
        client_type,
        turn_number: d.turn_number as i64,
        // Deltas
        delta_tool_calls: d.delta_tool_calls,
        delta_tool_failures: d.delta_tool_failures,
        delta_errors: d.delta_errors,
        delta_cancellations: d.delta_cancellations,
        delta_regenerations: d.delta_regenerations,
        delta_compactions: d.delta_compactions,
        delta_edit_and_retries: d.delta_edit_and_retries,
        delta_positive_ratings: d.delta_positive_ratings,
        delta_negative_ratings: d.delta_negative_ratings,
        delta_assistant_messages: d.delta_assistant_messages,
        delta_long_pauses: d.delta_long_pauses,
        delta_successful_tool_uses: d.delta_successful_tool_uses,
        // Turn-level snapshot values
        consecutive_cancellations: d.consecutive_cancellations as i64,
        // Turn-level absolute values
        time_to_first_token_ms: d.last_time_to_first_token_ms.map(|v| v as i64),
        total_response_time_ms: d.last_total_response_time_ms.map(|v| v as i64),
        // Per-turn ITL (delta uses u64, wire type uses i64)
        itl_p50_ms: d.last_itl_p50_ms.map(|v| v as i64),
        itl_p99_ms: d.last_itl_p99_ms.map(|v| v as i64),
        itl_max_ms: d.last_itl_max_ms.map(|v| v as i64),
        itl_mean_ms: d.last_itl_mean_ms.map(|v| v as i64),
        context_window_usage: c.context_window_usage as i64,
        model_id: c.primary_model_id.clone(),
        turn_duration_ms,
        turn_outcome,
        model_fingerprint,
        tools_used_this_turn: d.tools_this_turn.clone(),
        error_types_this_turn: d.error_types_this_turn.clone(),
        tool_outcomes: if d.tool_outcomes_this_turn.is_empty() {
            String::new()
        } else {
            serde_json::to_string(&d.tool_outcomes_this_turn).unwrap_or_default()
        },
        // Cumulative totals
        cumulative_tool_calls: c.tool_call_count as i64,
        cumulative_errors: c.error_count as i64,
        session_duration_seconds: c.session_duration_seconds as i64,
        total_tokens_before_compaction: c.total_tokens_before_compaction as i64,
        metadata,
        request_id,
        session_start_at: None, // set by caller if available
        feedback_requests_sent: feedback_requests_sent as i64,
        last_feedback_request_at,
        response_tokens: d.response_tokens.map(|v| v as i64),
        thinking_tokens: d.thinking_tokens.map(|v| v as i64),
        // LOC Attribution
        delta_agent_lines_added: d.delta_agent_lines_added,
        delta_agent_lines_removed: d.delta_agent_lines_removed,
        delta_agent_lines_added_reverted: d.delta_agent_lines_added_reverted,
        delta_agent_lines_removed_reverted: d.delta_agent_lines_removed_reverted,
        delta_human_lines_added: d.delta_human_lines_added,
        delta_human_lines_removed: d.delta_human_lines_removed,
        delta_human_lines_added_reverted: d.delta_human_lines_added_reverted,
        delta_human_lines_removed_reverted: d.delta_human_lines_removed_reverted,
        delta_agent_files_touched: d.delta_agent_files_touched,
        delta_human_files_touched: d.delta_human_files_touched,
        delta_total_files_touched: d.delta_total_files_touched,
        loc_tracking_enabled,
    }
}
