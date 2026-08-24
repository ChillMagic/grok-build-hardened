// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Privacy-build replacement for the upstream updater.
//!
//! The upstream implementation is intentionally deleted.  This module keeps
//! only the small API surface still referenced by the UI so the binary cannot
//! check, download, install, relaunch, or switch update channels.

use anyhow::Result;

use crate::version::{UpdateConfig, get_installed_grok_version};

pub use xai_grok_telemetry::events::CliUpdateTrigger;

#[derive(Clone, Copy, Debug)]
pub enum UpdateRunMode {
    Blocking,
    NonBlocking,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStatus {
    pub current_version: String,
    pub latest_version: Option<String>,
    pub update_available: bool,
    pub installer: Option<String>,
    pub channel: String,
    pub auto_update: Option<bool>,
    pub error: Option<String>,
}

pub fn print_update_status(status: &UpdateStatus, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string(status)?);
    } else {
        println!("Grok Build privacy fork - v{}", status.current_version);
        println!("{}", crate::UPDATER_REMOVED_MESSAGE);
    }
    Ok(())
}

pub async fn check_update_status(update_config: &UpdateConfig) -> UpdateStatus {
    UpdateStatus {
        current_version: get_installed_grok_version(),
        latest_version: None,
        update_available: false,
        installer: None,
        channel: update_config.channel.clone(),
        auto_update: Some(false),
        error: Some(crate::UPDATER_REMOVED_MESSAGE.to_owned()),
    }
}

pub async fn auto_update_target(_update_config: &UpdateConfig) -> Option<(&'static str, String)> {
    None
}

#[derive(Debug)]
pub struct EnsureLatestOutcome {
    pub installed: Option<String>,
    pub relaunch_needed: bool,
}

pub async fn ensure_latest_on_disk(_update_config: &UpdateConfig) -> Result<EnsureLatestOutcome> {
    Ok(EnsureLatestOutcome {
        installed: None,
        relaunch_needed: false,
    })
}

pub async fn get_installer() -> Option<&'static str> {
    None
}

#[derive(Debug, Clone)]
pub struct UpdateAvailable {
    pub latest_version: String,
}

/// Compatibility handle with no process behind it.
pub struct DisabledDownload;

impl DisabledDownload {
    pub async fn wait(&mut self) -> std::io::Result<std::process::ExitStatus> {
        Err(std::io::Error::other(crate::UPDATER_REMOVED_MESSAGE))
    }
}

pub struct BackgroundUpdateCheck {
    pub update: Option<UpdateAvailable>,
    pub download: Option<DisabledDownload>,
}

pub async fn check_update_background(_update_config: &UpdateConfig) -> BackgroundUpdateCheck {
    BackgroundUpdateCheck {
        update: None,
        download: None,
    }
}

pub async fn run_update_if_available(
    _run_mode: UpdateRunMode,
    _interactive: bool,
    _trigger: CliUpdateTrigger,
    _update_config: &UpdateConfig,
) -> Result<bool> {
    Ok(false)
}

pub fn restart_grok() -> Result<()> {
    anyhow::bail!(crate::UPDATER_REMOVED_MESSAGE)
}

pub async fn run_install_script(
    _installer: &str,
    _target: Option<&str>,
    _update_config: &UpdateConfig,
    _trigger: CliUpdateTrigger,
) -> Result<()> {
    anyhow::bail!(crate::UPDATER_REMOVED_MESSAGE)
}

#[doc(hidden)]
pub async fn download_with_progress(_url: &str, _dest: &std::path::Path) -> Result<()> {
    anyhow::bail!(crate::UPDATER_REMOVED_MESSAGE)
}

#[doc(hidden)]
pub async fn download_silent(_url: &str, _dest: &std::path::Path) -> Result<()> {
    anyhow::bail!(crate::UPDATER_REMOVED_MESSAGE)
}

pub async fn install_internal_from_bases(
    _target: Option<&str>,
    _update_config: &UpdateConfig,
    _bases: &[&str],
) -> Result<String> {
    anyhow::bail!(crate::UPDATER_REMOVED_MESSAGE)
}

pub async fn install_internal_from_base(
    _target: Option<&str>,
    _update_config: &UpdateConfig,
    _base: &str,
) -> Result<String> {
    anyhow::bail!(crate::UPDATER_REMOVED_MESSAGE)
}

#[doc(hidden)]
pub fn install_npm_for_test(
    _target: Option<&str>,
    _channel: &str,
    _npm_registry: Option<&str>,
) -> Result<()> {
    anyhow::bail!(crate::UPDATER_REMOVED_MESSAGE)
}

pub fn classify_install_error(
    _err: &anyhow::Error,
) -> xai_grok_telemetry::events::CliUpdateErrorKind {
    xai_grok_telemetry::events::CliUpdateErrorKind::Other
}

pub async fn apply_channel_switch(
    _channel_switch: Option<&str>,
    _update_config: &mut UpdateConfig,
) {
}

pub async fn run_update(
    _force: bool,
    _pinned_version: Option<&str>,
    _channel_switch: Option<&str>,
    _update_config: &mut UpdateConfig,
    _trigger: CliUpdateTrigger,
) -> Result<Option<String>> {
    anyhow::bail!(crate::UPDATER_REMOVED_MESSAGE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn every_update_entry_point_is_inert() {
        let cfg = UpdateConfig::privacy_default();
        assert!(get_installer().await.is_none());
        assert!(auto_update_target(&cfg).await.is_none());
        assert!(
            !run_update_if_available(
                UpdateRunMode::Blocking,
                true,
                CliUpdateTrigger::UserCommand,
                &cfg,
            )
            .await
            .unwrap()
        );
        assert!(
            run_update(
                true,
                Some("999.0.0"),
                Some("alpha"),
                &mut cfg.clone(),
                CliUpdateTrigger::UserCommand,
            )
            .await
            .is_err()
        );
    }
}
