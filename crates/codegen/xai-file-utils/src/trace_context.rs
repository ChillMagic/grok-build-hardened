// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Local-only tracing compatibility facade for the privacy build.
//!
//! Distributed trace propagation was removed with the upload/telemetry stack.
//! These functions intentionally never create or consume `traceparent` data and
//! never add tracing metadata to an outbound request.

/// Distributed trace identifiers are unavailable in the privacy build.
pub fn current_traceparent() -> Option<String> {
    None
}

/// Keep the ACP callback surface without accepting a remote trace parent.
pub fn span_from_meta_traceparent(
    _meta: &serde_json::Map<String, serde_json::Value>,
) -> tracing::Span {
    tracing::Span::none()
}

/// Remote trace linkage is intentionally disabled.
pub fn link_current_span_to_meta(_meta: &serde_json::Value) {}

#[cfg(test)]
mod tests {
    #[test]
    fn distributed_trace_context_is_permanently_disabled() {
        assert!(super::current_traceparent().is_none());
        assert!(super::span_from_meta_traceparent(&serde_json::Map::new()).is_none());
    }
}
