// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Removed feedback/session archive builder.

pub(crate) struct ArchiveCaps {
    pub(crate) archive_bytes: u64,
    pub(crate) file_bytes: u64,
}

pub(crate) const FEEDBACK_ARCHIVE_CAPS: ArchiveCaps = ArchiveCaps {
    archive_bytes: 0,
    file_bytes: 0,
};

#[derive(Debug, thiserror::Error)]
pub(crate) enum ArchiveError {
    #[error("feedback archive creation was removed from this privacy build")]
    Empty,
}

pub(crate) fn build_session_archive(
    _session_dir: &std::path::Path,
    _session_id: &str,
) -> Result<Vec<u8>, ArchiveError> {
    Err(ArchiveError::Empty)
}
