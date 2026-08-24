//! Local-only changelog reader for the privacy build.
//!
//! The upstream manager downloads release notes from a CDN at startup. Remote
//! content retrieval is removed here so a server cannot use changelog data as
//! a control or messaging channel. Existing local cache files remain readable.

use std::path::PathBuf;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ChangelogEntry {
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub breaking_change: bool,
}

pub struct Changelog {
    pub markdown: Option<String>,
    pub entries: Option<Vec<ChangelogEntry>>,
}

pub struct ChangelogManager {
    md_cache: PathBuf,
    json_cache: PathBuf,
}

impl Default for ChangelogManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ChangelogManager {
    pub fn new() -> Self {
        Self::from_env_home()
    }

    fn from_env_home() -> Self {
        let home = std::env::var_os("GROK_HOME")
            .map(PathBuf::from)
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(crate::util::grok_home::grok_home);
        Self {
            md_cache: home.join("CHANGELOG.md"),
            json_cache: home.join("CHANGELOG.json"),
        }
    }

    /// Read only files already present under the local Grok home. No DNS or
    /// HTTP operation is reachable from this implementation.
    pub fn fetch(&self) -> Changelog {
        let live = Self::from_env_home();
        Changelog {
            markdown: read_cache(&live.md_cache),
            entries: read_cache(&live.json_cache).and_then(|json| serde_json::from_str(&json).ok()),
        }
    }
}

fn read_cache(path: &std::path::Path) -> Option<String> {
    std::fs::read_to_string(path)
        .ok()
        .filter(|content| !content.trim().is_empty())
}

fn strip_markdown_inline(value: &str) -> String {
    value.replace("**", "").replace('`', "")
}

pub fn bullets_from_entries(entries: &[ChangelogEntry], max: usize) -> Vec<String> {
    entries
        .iter()
        .filter(|entry| !entry.description.is_empty())
        .take(max)
        .map(|entry| strip_markdown_inline(&entry.description))
        .collect()
}
