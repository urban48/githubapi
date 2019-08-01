use reqwest::Client;
use serde::Deserialize;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Error as ReqwestError;
use serde_json::error::Error as JsonError;

pub struct GitHubApi {
    username: String,
    password: String,
}

impl GitHubApi {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }


    fn api_get_call(
        &self,
        method: &str,
    ) -> Result<(String, Option<LimitRemainingReset>), GitHubApiError> {
        let url = format!("https://api.github.com/{}", method);

        let result = Client::new()
            .get(&url)
            .header("Accept", "application/vnd.github.v3+json")
            .basic_auth(&self.username, Some(&self.password))
            .send();

        match result {
            Ok(mut response) => {
                let headers = response.headers();

                let limit_remaining_reset = get_limits_from_headers(headers);

                match response.text() {
                    Ok(text) => Ok((text, limit_remaining_reset)),
                    Err(error) => Err(GitHubApiError::ReqwestError(error)),
                }
            }
            Err(error) => Err(GitHubApiError::ReqwestError(error)),
        }
    }

    pub fn get_rate_limit(&self) -> Result<ApiResponse<RateLimitResponse>, GitHubApiError> {
        let (text, limit_remaining_reset) = self.api_get_call("rate_limit")?;

        Ok(ApiResponse {
            result: parse_json(&text)?,
            limits: limit_remaining_reset,
        })
    }

    pub fn get_tags(
        &self,
        owner: &str,
        repository: &str,
    ) -> Result<ApiResponse<Vec<TagsResponse>>, GitHubApiError> {
        let method = format!("repos/{}/{}/tags", owner, repository);
        let (text, limit_remaining_reset) = self.api_get_call(&method)?;

        Ok(ApiResponse {
            result: parse_json(&text)?,
            limits: limit_remaining_reset,
        })
    }

    pub fn get_releases(
        &self,
        owner: &str,
        repository: &str,
    ) -> Result<ApiResponse<Vec<ReleasesResponse>>, GitHubApiError> {
        let method = format!("repos/{}/{}/releases", owner, repository);
        let (text, limit_remaining_reset) = self.api_get_call(&method)?;

        Ok(ApiResponse {
            result: parse_json(&text)?,
            limits: limit_remaining_reset,
        })
    }

    /// Fetches all pull requests for a repository.
    pub fn get_pull_requests(
        &self,
        owner: &str,
        repository: &str,
    ) -> Result<ApiResponse<Vec<PullRequestResponse>>, GitHubApiError> {
        let method = format!("repos/{}/{}/pulls", owner, repository);
        let (text, limit_remaining_reset) = self.api_get_call(&method)?;

        Ok(ApiResponse {
            result: parse_json(&text)?,
            limits: limit_remaining_reset,
        })
    }
}

// region Helpers

fn get_limits_from_headers(headers: &HeaderMap<HeaderValue>) -> Option<LimitRemainingReset> {
    let limit = get_u64_from_headers(headers, "x-ratelimit-limit")?;
    let remaining = get_u64_from_headers(headers, "x-ratelimit-remaining")?;
    let reset = get_u64_from_headers(headers, "x-ratelimit-reset")?;

    Some(LimitRemainingReset {
        limit,
        remaining,
        reset,
    })
}

fn get_u64_from_headers(headers: &HeaderMap, key: &str) -> Option<u64> {
    match headers.get(key) {
        Some(header_value) => match header_value.to_str() {
            Ok(string_value) => match string_value.parse() {
                Ok(value) => Some(value),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

fn parse_json<'a, T>(text: &'a str) -> Result<T, GitHubApiError>
    where
        T: Deserialize<'a>,
{
    match serde_json::from_str(&text) {
        Ok(value) => Ok(value),
        Err(error) => Err(GitHubApiError::JsonError(error)),
    }
}

// endregion

// region Envelopes

#[derive(Debug)]
pub struct ApiResponse<T> {
    result: T,
    limits: Option<LimitRemainingReset>,
}

#[derive(Debug)]
pub enum GitHubApiError {
    NotImplemented,
    JsonError(JsonError),
    ReqwestError(ReqwestError),
}

// endregion

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
    pub limit: u64,
    pub remaining: u64,
    pub reset: u64,
}

// endregion

// region TagsResponse

#[derive(Debug, Deserialize)]
pub struct TagsResponse {
    name: String,
    zipball_url: String,
    tarball_url: String,
    commit: TagsCommit,
    node_id: String,
}

#[derive(Debug, Deserialize)]
pub struct TagsCommit {
    sha: String,
    url: String,
}

// endregion

// region ReleasesResponse

#[derive(Debug, Deserialize)]
pub struct ReleasesResponse {
    url: String,
    assets_url: String,
    upload_url: String,
    html_url: String,
    id: u64,
    node_id: String,
    tag_name: String,
    target_commitish: String,
    name: String,
    draft: bool,
    author: ReleasesPerson,
    prerelease: bool,
    created_at: String,
    published_at: String,
    assets: Vec<ReleasesAsset>,
    tarball_url: String,
    zipball_url: String,
    body: String,
}

#[derive(Debug, Deserialize)]
pub struct ReleasesAsset {
    url: String,
    id: u64,
    node_id: String,
    name: String,
    label: String,
    uploader: ReleasesPerson,
    content_type: String,
    state: String,
    size: u64,
    download_count: u64,
    created_at: String,
    updated_at: String,
    browser_download_url: String,
}

#[derive(Debug, Deserialize)]
pub struct ReleasesPerson {
    login: String,
    id: u64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    r#type: String,
    site_admin: bool,
}

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
