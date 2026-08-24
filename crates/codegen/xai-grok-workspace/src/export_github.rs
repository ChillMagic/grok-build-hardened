// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Fail-closed replacement for the upstream repository-to-GitHub exporter.
//!
//! The original module initialized a repository, committed the project tree,
//! and executed `git push`. That whole implementation is removed so neither a
//! model tool call nor a workspace RPC can publish a repository.

use std::path::Path;

use xai_grok_workspace_types::rpc::export_github::{ExportGithubError, ExportGithubResponse};

pub struct ExportGithubParams<'a> {
    pub project_dir: &'a Path,
    pub repo_full_name: Option<&'a str>,
    pub remote_url_base: &'a str,
    pub web_url_base: &'a str,
    pub branch: Option<&'a str>,
    pub commit_message: Option<&'a str>,
}

#[derive(Debug)]
pub struct ExportGithubFailure {
    pub kind: ExportGithubError,
    pub message: String,
}

pub async fn run_export(
    _params: ExportGithubParams<'_>,
) -> Result<ExportGithubResponse, ExportGithubFailure> {
    Err(ExportGithubFailure {
        kind: ExportGithubError::PushRejected,
        message: "repository export was removed from this privacy build".to_string(),
    })
}
