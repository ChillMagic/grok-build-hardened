// Added by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Compile-time privacy invariants for the no-upload/no-cloud-control fork.
//!
//! These values deliberately have no environment, config, CLI, managed, or
//! remote override.  Normal authentication, model inference, and explicitly
//! selected media generation remain available; passive/background export and
//! server-controlled behavior do not.

/// Marker used by audits and the installed launcher's binary-string check.
pub const PRIVACY_BUILD: bool = true;

/// Remote settings, managed policy sync, campaigns, and remote version policy
/// are not accepted by this build.
pub const REMOTE_CONTROL_COMPILED_IN: bool = false;

/// Repository/session/trace/telemetry/feedback/share uploads are not accepted
/// by this build.  Model prompts still travel to the selected inference API.
pub const PASSIVE_UPLOADS_COMPILED_IN: bool = false;

/// Stable message returned by disabled high-level operations.
pub const REMOVED_MESSAGE: &str = "disabled by the no-upload/no-cloud-control privacy build";

/// ACP extensions that either upload local/session data or expose remote
/// workspace control.  Kept in one exhaustive choke point so adding a new
/// branch to the large extension dispatcher cannot accidentally re-arm an
/// existing family by changing its handler.
pub fn blocks_extension(method: &str) -> bool {
    method.starts_with("x.ai/cloud/")
        || method.starts_with("x.ai/bundle/")
        || method.starts_with("x.ai/review")
        || matches!(
            method,
            "x.ai/workspaces/list"
                | "x.ai/feedback"
                | "x.ai/feedback/dismiss"
                | "x.ai/feedback/upload-trace"
                | "x.ai/btw"
                | "x.ai/share_session"
                | "x.ai/privacy/setCodingDataRetention"
                | "x.ai/consent/record"
                | "x.ai/rollout/survey"
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn privacy_invariants_are_compile_time_off() {
        assert!(PRIVACY_BUILD);
        assert!(!REMOTE_CONTROL_COMPILED_IN);
        assert!(!PASSIVE_UPLOADS_COMPILED_IN);
    }

    #[test]
    fn upload_and_cloud_extensions_are_blocked_but_inference_is_not() {
        for method in [
            "x.ai/cloud/env/create",
            "x.ai/feedback",
            "x.ai/share_session",
            "x.ai/consent/record",
        ] {
            assert!(blocks_extension(method), "{method}");
        }
        assert!(!blocks_extension("x.ai/models/list"));
        assert!(!blocks_extension("x.ai/session/state"));
    }
}
