use serde::Deserialize;
use reqwest::Client;

use reqwest::Error as ReqwestError;
use serde_json::error::Error as JsonError;

pub struct GitHubApi {
    username: String,
    password: String,
}

#[derive(Debug)]
pub enum GitHubApiError {
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
                }
                Err(error) => Err(GitHubApiError::ReqwestError(error))
            }
    }

    pub fn get_pull_requests(&self, owner: &str, repository: &str) -> Result<Vec<PullRequest>, GitHubApiError> {
        let method = format!("repos/{}/{}/pulls", owner, repository);

        let result = self.api_get_call(&method)?;

        match serde_json::from_str(&result) {
            Ok(value) => Ok(value),
            Err(error) => Err(GitHubApiError::JsonError(error))
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum OpenClosed {
    #[serde(rename(deserialize = "open"))]
    Open,
    #[serde(rename(deserialize = "closed"))]
    Closed,
}

#[derive(Debug, Deserialize)]
pub struct PullRequest {
    url: String,
    id: u32,
    title: String,
    state: OpenClosed,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
