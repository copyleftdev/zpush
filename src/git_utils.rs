use git2::Repository;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    #[error("Git error: {0}")]
    GitError(#[from] git2::Error),
    #[error("Remote URL not found in repository config.")]
    RemoteNotFound,
    #[error("Failed to parse remote URL: {0}")]
    ParseError(String),
}

pub fn get_origin_repo() -> Result<(String, String), GitError> {
    let repo = Repository::open(".")?;
    let remote = repo.find_remote("origin")?;
    let url = remote.url().ok_or_else(|| GitError::ParseError("Remote URL is not valid UTF-8.".to_string()))?;
    let (owner, repo_name) = if url.starts_with("git@") {
        let path = url.trim_start_matches("git@github.com:");
        let path = path.strip_suffix(".git").unwrap_or(path);
        let mut parts = path.split('/');
        let owner = parts.next().ok_or_else(|| GitError::ParseError("Cannot parse owner from URL.".to_string()))?;
        let repo_name = parts.next().ok_or_else(|| GitError::ParseError("Cannot parse repo name from URL.".to_string()))?;
        (owner.to_string(), repo_name.to_string())
    } else if url.starts_with("https://") {
        let path = url.trim_start_matches("https://github.com/");
        let path = path.strip_suffix(".git").unwrap_or(path);
        let mut parts = path.split('/');
        let owner = parts.next().ok_or_else(|| GitError::ParseError("Cannot parse owner from URL.".to_string()))?;
        let repo_name = parts.next().ok_or_else(|| GitError::ParseError("Cannot parse repo name from URL.".to_string()))?;
        (owner.to_string(), repo_name.to_string())
    } else {
        return Err(GitError::ParseError("Unsupported remote URL format.".to_string()));
    };
    Ok((owner, repo_name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::Repository;
    use tempfile::tempdir;
    use std::env;

    #[test]
    fn test_get_origin_repo() {
        let dir = tempdir().unwrap();
        let repo = Repository::init(dir.path()).unwrap();
        repo.remote("origin", "git@github.com:owner/repo.git").unwrap();
        let orig_dir = env::current_dir().unwrap();
        env::set_current_dir(dir.path()).unwrap();
        let res = get_origin_repo().unwrap();
        assert_eq!(res, ("owner".to_string(), "repo".to_string()));
        env::set_current_dir(orig_dir).unwrap();
    }
}
