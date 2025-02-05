use reqwest::Client;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitHubError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("GitHub responded with error: {0}")]
    ResponseError(String),
}

pub struct GitHubClient {
    client: Client,
    token: String,
    user_agent: String,
}

impl GitHubClient {
    pub fn new(token: String) -> Self {
        GitHubClient {
            client: Client::new(),
            token,
            user_agent: "zenvpush".to_string(),
        }
    }

    pub async fn verify_token(&self) -> Result<(), GitHubError> {
        let response = self
            .client
            .get("https://api.github.com/user")
            .header("User-Agent", &self.user_agent)
            .bearer_auth(&self.token)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(GitHubError::ResponseError(format!("GitHub API responded with status: {}", response.status())))
        }
    }

    pub async fn push_secret(
        &self,
        repo: &str,
        key: &str,
        encrypted_value: &str,
        key_id: &str,
    ) -> Result<(), GitHubError> {
        let url = format!("https://api.github.com/repos/{}/actions/secrets/{}", repo, key);
        let body = json!({
            "encrypted_value": encrypted_value,
            "key_id": key_id
        });
        let response = self
            .client
            .put(&url)
            .header("User-Agent", &self.user_agent)
            .bearer_auth(&self.token)
            .json(&body)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(GitHubError::ResponseError(format!("Failed to push secret {}: HTTP {}", key, response.status())))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_verify_token_fail() {
        let client = GitHubClient::new("dummy-token".to_string());
        let res = client.verify_token().await;
        assert!(res.is_err());
    }
}
