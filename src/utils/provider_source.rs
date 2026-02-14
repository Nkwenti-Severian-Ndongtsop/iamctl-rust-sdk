use std::fmt;

use crate::utils::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GithubProviderSource {
    pub owner: String,
    pub repo: String,
    pub subdir: Option<String>,
}

impl GithubProviderSource {
    pub fn parse(input: &str) -> Result<Self> {
        // Expected formats:
        // - github:OWNER/REPO
        // - github:OWNER/REPO//subdir
        let rest = input
            .strip_prefix("github:")
            .ok_or_else(|| Error::Config(format!("Unsupported provider source: {input}")))?;

        let (repo_part, subdir) = match rest.split_once("//") {
            Some((repo_part, subdir)) if !subdir.trim().is_empty() => {
                (repo_part, Some(subdir.trim().trim_matches('/').to_string()))
            }
            Some((repo_part, _)) => (repo_part, None),
            None => (rest, None),
        };

        let (owner, repo) = repo_part
            .split_once('/')
            .ok_or_else(|| Error::Config(format!("Invalid github provider source: {input}")))?;

        let owner = owner.trim();
        let repo = repo.trim();
        if owner.is_empty() || repo.is_empty() {
            return Err(Error::Config(format!(
                "Invalid github provider source (empty owner or repo): {input}"
            )));
        }

        Ok(Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            subdir,
        })
    }

    pub fn github_repo_slug(&self) -> String {
        format!("{}/{}", self.owner, self.repo)
    }
}

impl fmt::Display for GithubProviderSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(subdir) = &self.subdir {
            write!(f, "github:{}/{}//{}", self.owner, self.repo, subdir)
        } else {
            write!(f, "github:{}/{}", self.owner, self.repo)
        }
    }
}

pub fn derive_github_release_tag(provider: &str, version: &str) -> Result<String> {
    let provider = provider.trim();
    let version = version.trim();

    if provider.is_empty() {
        return Err(Error::Config("Provider name cannot be empty".to_string()));
    }
    if version.is_empty() {
        return Err(Error::Config(
            "Provider version cannot be empty".to_string(),
        ));
    }

    // We intentionally do not validate SemVer here; we only normalize the leading 'v'.
    let version = version.strip_prefix('v').unwrap_or(version);
    Ok(format!("{}-v{}", provider, version))
}
