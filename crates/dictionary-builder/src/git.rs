//! Git integration functionality.

use std::path::Path;

use git2::{Repository, StatusOptions};
use tracing::{debug, warn};

use crate::Result;

/// Get the current commit hash of a repository
pub fn get_current_commit_hash(repo_path: &Path) -> Result<String> {
    let repo = Repository::open(repo_path)?;

    // Check if there are uncommitted changes
    let mut status_opts = StatusOptions::new();
    status_opts.include_untracked(true);
    let statuses = repo.statuses(Some(&mut status_opts))?;

    if !statuses.is_empty() {
        warn!(
            "Repository has {} uncommitted changes. The generated manifest may not match the actual state.",
            statuses.len()
        );
    }

    let head = repo.head()?;
    let commit = head.peel_to_commit()?;
    let hash = commit.id().to_string();

    debug!("Current commit hash: {}", hash);
    Ok(hash)
}
