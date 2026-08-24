// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Removed external-record mapping facade.
//!
//! Upstream converted prompts, tool details, session metadata, and product
//! events into exportable records here.  Those mappings are deleted.  The
//! zero-sized compatibility type and typed functions make every event resolve
//! to `None`, even if an old call site remains.

use crate::events;

#[derive(Debug, Clone, Default)]
pub struct ExternalRecord;

macro_rules! removed_mapper {
    ($name:ident, $event:ty) => {
        pub fn $name(_event: &$event) -> Option<ExternalRecord> {
            None
        }
    };
}

removed_mapper!(map_session_start, events::SessionHarness);
removed_mapper!(map_session_new, events::SessionNew);
removed_mapper!(map_session_end, events::SessionEnded);
removed_mapper!(map_user_prompt, events::PromptSubmitted);
removed_mapper!(map_turn_completed, events::TurnCompleted);
removed_mapper!(map_api_request, events::ModelResponseReceived);
removed_mapper!(map_rate_limit_hit, events::RateLimitHit);
removed_mapper!(map_api_error, events::ApiError);
removed_mapper!(map_tool_result, events::ToolCallCompleted);
removed_mapper!(map_tool_decision, events::PermissionDecisionPayload);
removed_mapper!(map_mcp_server_connected, events::McpServerConnected);
removed_mapper!(map_mcp_server_failed, events::McpServerFailed);
removed_mapper!(map_plan_mode_toggled, events::PlanModeToggled);
removed_mapper!(map_contextual_tip, events::ContextualTip);
removed_mapper!(map_yolo_toggled, events::YoloToggled);
removed_mapper!(map_skill_activated, events::SkillDispatched);
removed_mapper!(map_plugin_installed, events::PluginInstalled);
removed_mapper!(map_plugin_used, events::PluginUsed);
removed_mapper!(map_compaction, events::CompactionCompleted);
removed_mapper!(map_subagent_launched, events::SubagentLaunched);
removed_mapper!(map_subagent_completed, events::SubagentCompleted);
removed_mapper!(map_auth, events::Login);
removed_mapper!(map_internal_error, events::InternalError);
removed_mapper!(map_agent_connect, events::AgentConnect);
removed_mapper!(map_startup_complete, events::StartupComplete);
removed_mapper!(map_model_switched, events::ModelSwitched);

#[cfg(test)]
mod tests {
    #[test]
    fn external_record_carries_no_data() {
        assert_eq!(std::mem::size_of::<super::ExternalRecord>(), 0);
    }
}
