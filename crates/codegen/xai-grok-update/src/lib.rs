pub mod auto_update;
pub mod version;
mod version_policy;

/// The private build contains no callable self-update path. Updating means a
/// fresh source review, rebuild, and explicit installation outside the binary.
pub const UPDATER_COMPILED_IN: bool = false;
pub const UPDATER_REMOVED_MESSAGE: &str =
    "updating is disabled in this privacy build; install a newly audited source build manually";

pub use auto_update::UpdateStatus;
pub use version::{UpdateConfig, channel_label, channel_name, write_version_cache};
pub use version_policy::enforce_version_policy_or_exit;
