use reqwest::Client;
use serde::Deserialize;

use reqwest::Error as ReqwestError;
use serde_json::error::Error as JsonError;

pub struct GitHubApi {
    username: String,
    password: String,
}

#[derive(Debug)]
pub enum GitHubApiError {
    NotImplemented,
    JsonError(JsonError),
    ReqwestError(ReqwestError),
}

impl GitHubApi {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    fn api_get_call(&self, method: &str) -> Result<String, GitHubApiError> {
        let url = format!("https://api.github.com/{}", method);

        match Client::new()
            .get(&url)
            .header("Accept", "application/vnd.github.v3+json")
            .basic_auth(&self.username, Some(&self.password))
            .send()
        {
            Ok(mut response) => match response.text() {
                Ok(text) => Ok(text),
                Err(error) => Err(GitHubApiError::ReqwestError(error)),
            },
            Err(error) => Err(GitHubApiError::ReqwestError(error)),
        }
    }

    pub fn get_rate_limit(&self) -> Result<RateLimitResponse, GitHubApiError> {
        let result = self.api_get_call("rate_limit")?;

        match serde_json::from_str(&result) {
            Ok(value) => Ok(value),
            Err(error) => Err(GitHubApiError::JsonError(error)),
        }
    }

    pub fn get_tags(&self) -> Result<TagsResponse, GitHubApiError> {
        Err(GitHubApiError::NotImplemented)
    }

    pub fn get_repos(&self) -> Result<ReposResponse, GitHubApiError> {
        Err(GitHubApiError::NotImplemented)
    }

    /// Fetches all pull requests for a repository.
    pub fn get_pull_requests(
        &self,
        owner: &str,
        repository: &str,
    ) -> Result<Vec<PullRequestResponse>, GitHubApiError> {
        let method = format!("repos/{}/{}/pulls", owner, repository);
        let result = self.api_get_call(&method)?;

        match serde_json::from_str(&result) {
            Ok(value) => Ok(value),
            Err(error) => Err(GitHubApiError::JsonError(error)),
        }
    }
}

// region Enums

#[derive(Debug, Deserialize)]
pub enum OpenClosed {
    #[serde(rename(deserialize = "open"))]
    Open,
    #[serde(rename(deserialize = "closed"))]
    Closed,
}

// endregion

// region PullRequestResponse

#[derive(Debug, Deserialize)]
pub struct PullRequestResponse {
    url: String,
    id: u32,
    title: String,
    state: OpenClosed,
}

// endregion

// region RateLimitResponse

#[derive(Debug, Deserialize)]
pub struct RateLimitResponse {
    pub resources: RateLimitResources,
    pub rate: LimitRemainingReset,
}

#[derive(Debug, Deserialize)]
pub struct RateLimitResources {
    pub core: LimitRemainingReset,
    pub search: LimitRemainingReset,
    pub graphql: LimitRemainingReset,
    pub integration_manifest: LimitRemainingReset,
}

#[derive(Debug, Deserialize)]
pub struct LimitRemainingReset {
    pub limit: u32,
    pub remaining: u32,
    pub reset: u64,
}

// endregion

// region TagsResponse

#[derive(Debug, Deserialize)]
pub struct TagsResponse {}

// endregion

// region ReposResponse

#[derive(Debug, Deserialize)]
pub struct ReposResponse {}

// endregion

// region Tests

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// endregion
