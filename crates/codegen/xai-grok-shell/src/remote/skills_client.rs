// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Removed server-delivered product Skills catalog.
//!
//! Local user-installed skills remain available through local discovery. This
//! facade exists only so callers compiled against the upstream API fail closed.

use std::sync::Arc;

use serde::Deserialize;
use xai_grok_tools::implementations::skills::types::SkillInfo;

use crate::auth::AuthManager;

pub const CHAT_PRODUCT_META_VALUE: &str = "chat";
pub const CHAT_PRODUCT_META_KEY: &str = "product";
pub(crate) const REMOTE_SKILLS_COMPILED_IN: bool = false;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundledSkill {
    #[serde(default)]
    pub index: i32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub display_name: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListBundledSkillsResponse {
    #[serde(default)]
    pub skills: Vec<BundledSkill>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSkill {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub skill_md_content: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListUserSkillsResponse {
    #[serde(default)]
    pub skills: Vec<UserSkill>,
}

#[derive(Debug, Clone, Default)]
pub struct ProductSkillsCatalog {
    pub bundled: Vec<BundledSkill>,
    pub user: Vec<UserSkill>,
    pub user_list_failed: bool,
}

impl ProductSkillsCatalog {
    pub(crate) fn to_skill_infos(&self) -> Vec<SkillInfo> {
        Vec::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SkillsError {
    #[error("server-delivered skills were removed from this privacy build")]
    NoAuth,
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("request failed: {status}")]
    Http { status: u16 },
    #[error("parse error: {0}")]
    Parse(#[from] serde_json::Error),
}

pub struct SkillsClient {
    _auth: Arc<AuthManager>,
}

impl SkillsClient {
    pub fn new(auth: Arc<AuthManager>) -> Self {
        Self { _auth: auth }
    }

    pub(crate) async fn try_list_catalog(
        &self,
        _locale: &str,
    ) -> Result<(ProductSkillsCatalog, bool), SkillsError> {
        Err(SkillsError::NoAuth)
    }

    pub async fn list_catalog(&self, _locale: &str) -> ProductSkillsCatalog {
        ProductSkillsCatalog::default()
    }

    #[cfg(test)]
    pub(crate) fn with_base_url(auth: Arc<AuthManager>, _base_url: impl Into<String>) -> Self {
        Self::new(auth)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn server_delivered_skills_are_disabled() {
        assert!(!super::REMOTE_SKILLS_COMPILED_IN);
        assert!(
            super::ProductSkillsCatalog::default()
                .to_skill_infos()
                .is_empty()
        );
    }
}
