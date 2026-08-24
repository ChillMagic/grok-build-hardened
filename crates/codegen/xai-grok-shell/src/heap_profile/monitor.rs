// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Inert heap-profile monitor for the privacy build.
//!
//! Upstream could create jemalloc dumps and upload the dump plus metadata.
//! This replacement contains neither dump orchestration nor an uploader and
//! forces profiling off regardless of local or server-provided settings.

use std::collections::BTreeSet;
use std::sync::Arc;
use std::time::Duration;

use crate::auth::AuthManager;
use crate::session::repo_changes::UploadMethod;

pub const HARD_DUMP_SIZE_CAP_BYTES: u64 = 128 * 1024 * 1024;
pub const SCOPED_KILL_SWITCH_INTERVAL: Duration = Duration::from_secs(5 * 60);

const DEFAULT_POLL_INTERVAL_SECS: u64 = 30;
const MIN_POLL_INTERVAL_SECS: u64 = 5;
const MAX_POLL_INTERVAL_SECS: u64 = 300;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JemallocHeapProfileConfig {
    pub enabled: bool,
    pub thresholds: Vec<u64>,
    pub poll_interval: Duration,
}

impl Default for JemallocHeapProfileConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            thresholds: Vec::new(),
            poll_interval: Duration::from_secs(DEFAULT_POLL_INTERVAL_SECS),
        }
    }
}

/// Empty marker: credentials, endpoints, and upload methods are not retained.
#[derive(Clone)]
pub struct HeapProfileUploadHandles;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DumpAttemptOutcome {
    Deferred,
    DumpFailed,
    DumpTimeout,
    SizeCap,
    UploadOk,
    UploadFailed,
}

pub fn should_latch(outcome: DumpAttemptOutcome) -> bool {
    !matches!(outcome, DumpAttemptOutcome::Deferred)
}

pub fn is_valid_session_id(session_id: &str) -> bool {
    uuid::Uuid::try_parse(session_id).is_ok()
}

pub fn sanitize_version(version: &str) -> String {
    let mut out = String::with_capacity(version.len());
    let mut previous_underscore = false;
    for character in version.chars() {
        let accepted = character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-');
        if accepted {
            out.push(character);
            previous_underscore = character == '_';
        } else if !previous_underscore {
            out.push('_');
            previous_underscore = true;
        }
    }
    let trimmed = out.trim_matches('_').to_owned();
    if trimmed.is_empty() {
        "unknown".to_owned()
    } else {
        trimmed
    }
}

/// Kept only for local filename compatibility; no code sends these objects.
pub fn object_paths(session_id: &str, version: &str, ts_unix: u64) -> (String, String) {
    let version = sanitize_version(version);
    let base = format!("{session_id}/jemalloc/{session_id}-{version}-{ts_unix}");
    (format!("{base}.heap"), format!("{base}.meta.json"))
}

pub fn normalize_thresholds(thresholds: impl IntoIterator<Item = u64>) -> Vec<u64> {
    let mut thresholds: Vec<u64> = thresholds.into_iter().collect();
    thresholds.sort_unstable();
    thresholds.dedup();
    thresholds
}

pub fn clamp_poll_interval_secs(seconds: Option<u64>) -> u64 {
    seconds
        .unwrap_or(DEFAULT_POLL_INTERVAL_SECS)
        .clamp(MIN_POLL_INTERVAL_SECS, MAX_POLL_INTERVAL_SECS)
}

pub fn resolve_jemalloc_heap_profile(
    _remote_enabled: Option<bool>,
    _remote_thresholds: Option<&[u64]>,
    remote_poll_interval_secs: Option<u64>,
    _data_collection_disabled: bool,
    _trace_upload_enabled: bool,
    _prof_available: bool,
) -> JemallocHeapProfileConfig {
    JemallocHeapProfileConfig {
        enabled: false,
        thresholds: Vec::new(),
        poll_interval: Duration::from_secs(clamp_poll_interval_secs(remote_poll_interval_secs)),
    }
}

pub struct HeapProfileMonitor {
    config: JemallocHeapProfileConfig,
    latched: BTreeSet<u64>,
}

impl Default for HeapProfileMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl HeapProfileMonitor {
    pub fn new() -> Self {
        Self {
            config: JemallocHeapProfileConfig::default(),
            latched: BTreeSet::new(),
        }
    }

    pub fn config(&self) -> &JemallocHeapProfileConfig {
        &self.config
    }

    pub fn latched(&self) -> &BTreeSet<u64> {
        &self.latched
    }

    pub fn session_id(&self) -> Option<&str> {
        None
    }

    pub fn upload_in_flight(&self) -> bool {
        false
    }

    pub(crate) fn clear_upload_in_flight(&mut self) {}

    pub fn reconfigure(
        &mut self,
        config: JemallocHeapProfileConfig,
        _upload_handles: Option<HeapProfileUploadHandles>,
    ) {
        self.config = JemallocHeapProfileConfig {
            enabled: false,
            thresholds: Vec::new(),
            poll_interval: config.poll_interval,
        };
        let _ = super::set_prof_active(false);
    }

    pub fn set_session_id(&mut self, _session_id: String) {}

    pub(crate) fn begin_tick(&mut self) -> Option<PendingDump> {
        None
    }

    pub(crate) fn finish_tick(&mut self, _threshold: u64, _outcome: DumpAttemptOutcome) {}

    pub async fn poll_tick(&mut self) {}
}

pub(crate) struct PendingDump {
    pub threshold: u64,
}

impl PendingDump {
    pub(crate) async fn execute(self) -> DumpAttemptOutcome {
        DumpAttemptOutcome::Deferred
    }
}

pub fn build_upload_handles(
    _auth_manager: Arc<AuthManager>,
    _bucket_url: Option<String>,
    _upload_method: UploadMethod,
) -> HeapProfileUploadHandles {
    HeapProfileUploadHandles
}
