//! Local-only version metadata for the privacy build.
//!
//! All upstream network version discovery and package-manager execution was
//! deleted with the updater implementation.

use anyhow::Result;

use xai_grok_shell::env::GrokBuildEnvironment;

#[derive(Debug, Clone)]
pub struct UpdateConfig {
    pub proxy_base_url: String,
    pub auth_scope: String,
    pub deployment_key: Option<String>,
    pub alpha_test_key: Option<String>,
    pub channel: String,
    pub npm_registry: Option<String>,
}

impl UpdateConfig {
    pub fn from_environment(env: &GrokBuildEnvironment) -> Self {
        Self {
            proxy_base_url: env.cli_chat_proxy_base_url(),
            auth_scope: xai_grok_shell::auth::GrokComConfig::default().auth_scope(),
            deployment_key: None,
            alpha_test_key: None,
            channel: "privacy".to_owned(),
            npm_registry: None,
        }
    }

    pub fn privacy_default() -> Self {
        Self::from_environment(&GrokBuildEnvironment::Production)
    }
}

fn removed<T>() -> Result<T> {
    anyhow::bail!(crate::UPDATER_REMOVED_MESSAGE)
}

#[doc(hidden)]
pub async fn fetch_npm_tag_for_test(_tag: &str, _npm_registry: Option<&str>) -> Result<String> {
    removed()
}

#[doc(hidden)]
pub async fn fetch_npm_version_for_test(
    _channel: &str,
    _npm_registry: Option<&str>,
) -> Result<String> {
    removed()
}

#[doc(hidden)]
pub async fn fetch_gh_release_version(_channel: &str) -> Result<String> {
    removed()
}

#[doc(hidden)]
pub async fn fetch_gcs_version_from_base(_channel: &str, _base_url: &str) -> Result<String> {
    removed()
}

pub async fn fetch_latest_version(_installer: &str, _config: &UpdateConfig) -> Result<String> {
    removed()
}

pub async fn get_latest_version(_installer: &str, _config: &UpdateConfig) -> Result<String> {
    removed()
}

pub async fn write_version_cache(_version: &str, _stable_version: Option<&str>) {}

pub async fn is_version_cache_fresh() -> bool {
    true
}

pub use xai_grok_version::installed as get_installed_grok_version;

pub fn installed_on_disk_version() -> Option<String> {
    None
}

pub fn cached_stable_version() -> Option<String> {
    None
}

pub fn channel_name() -> Option<&'static str> {
    Some("privacy")
}

pub fn channel_label() -> &'static str {
    " [privacy]"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn remote_version_sources_are_removed() {
        let cfg = UpdateConfig::privacy_default();
        assert!(fetch_latest_version("internal", &cfg).await.is_err());
        assert!(
            fetch_gcs_version_from_base("stable", "http://127.0.0.1:1")
                .await
                .is_err()
        );
        assert_eq!(channel_name(), Some("privacy"));
    }
}
