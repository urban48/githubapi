use lazy_static::lazy_static;

use reqwest::Client;
use serde::Deserialize;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Error as ReqwestError;
use serde_json::error::Error as JsonError;

use regex::Regex;

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
        page: u64,
    ) -> Result<(String, Option<LimitRemainingReset>, Option<u64>), GitHubApiError> {
        let url = format!("https://api.github.com/{}?page={}", method, page);

        let result = Client::new()
            .get(&url)
            .header("Accept", "application/vnd.github.v3+json")
            .basic_auth(&self.username, Some(&self.password))
            .send();

        match result {
            Ok(mut response) => {
                let headers = response.headers();
                let limit_remaining_reset = headers.get_rate_limits();

                let next_page = headers.get_next_page();

                match response.text() {
                    Ok(text) => Ok((text, limit_remaining_reset, next_page)),
                    Err(error) => Err(GitHubApiError::ReqwestError(error)),
                }
            }
            Err(error) => Err(GitHubApiError::ReqwestError(error)),
        }
    }

    pub fn get_rate_limit(&self) -> Result<ApiResponse<RateLimitResponse>, GitHubApiError> {
        let (text, limit_remaining_reset, _) = self.api_get_call("rate_limit", 1)?;

        Ok(ApiResponse {
            result: parse_json(&text)?,
            limits: limit_remaining_reset,
            owner: None,
            repository: None,
            next_page: None
        })
    }

    fn get_tags_page(
        &self,
        owner: &str,
        repository: &str,
        page: u64,
    ) -> Result<ApiResponse<Vec<TagsResponse>>, GitHubApiError> {
        let method = format!("repos/{}/{}/tags", owner, repository);
        let (text, limit_remaining_reset, next_page) = self.api_get_call(&method, page)?;

        Ok(ApiResponse {
            result: parse_json(&text)?,
            limits: limit_remaining_reset,
            owner: Some(owner.to_string()),
            repository: Some(repository.to_string()),
            next_page,
        })
    }

    pub fn get_tags(
        &self,
        owner: &str,
        repository: &str,
    ) -> Result<ApiResponse<Vec<TagsResponse>>, GitHubApiError> {
        self.get_tags_page(owner, repository, 1)
    }

    pub fn get_tags_next<T>(
        &self,
        previous: &ApiResponse<T>
    ) -> Result<ApiResponse<Vec<TagsResponse>>, GitHubApiError> {
        let owner = &previous.owner.clone().unwrap();
        let repository = &previous.repository.clone().unwrap();
        let next_page = previous.next_page.unwrap();

        self.get_tags_page(owner, repository, next_page)
    }

    pub fn get_releases(
        &self,
        owner: &str,
        repository: &str,
    ) -> Result<ApiResponse<Vec<ReleasesResponse>>, GitHubApiError> {
        let method = format!("repos/{}/{}/releases", owner, repository);
        let (text, limit_remaining_reset, next_page) = self.api_get_call(&method, 1)?;

        Ok(ApiResponse {
            result: parse_json(&text)?,
            limits: limit_remaining_reset,
            owner: Some(owner.to_string()),
            repository: Some(repository.to_string()),
            next_page,
        })
    }
}

// region Helpers

trait HeaderMapExtensions {
    fn get_as_u64(&self, key: &str) -> Option<u64>;

    fn get_rate_limits(&self) -> Option<LimitRemainingReset>;
    fn is_paginated(&self) -> bool;
    fn get_pagination(&self) -> Option<Vec<Pagination>>;
    fn get_next_page(&self) -> Option<u64>;
}

impl HeaderMapExtensions for HeaderMap<HeaderValue> {
    fn get_as_u64(&self, key: &str) -> Option<u64> {
        match self.get(key) {
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

    fn get_rate_limits(&self) -> Option<LimitRemainingReset> {
        let limit = self.get_as_u64("x-ratelimit-limit")?;
        let remaining = self.get_as_u64("x-ratelimit-remaining")?;
        let reset = self.get_as_u64("x-ratelimit-reset")?;

        Some(LimitRemainingReset {
            limit,
            remaining,
            reset,
        })
    }

    fn is_paginated(&self) -> bool {
        self.get("Link").is_some()
    }

    fn get_pagination(&self) -> Option<Vec<Pagination>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"<.*?\?page=(\d+)>; rel="(.*?)""#).unwrap();
        }

        match self.get("Link") {
            None => None,
            Some(header) => match header.to_str() {
                Ok(data) => Some(
                    RE.captures_iter(data)
                        .map(|it| {
                            (
                                it.get(2).unwrap().as_str(),
                                it.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                            )
                        })
                        .map(|(direction, number)| match direction {
                            "first" => Pagination::First(number),
                            "prev" => Pagination::Previous(number),
                            "next" => Pagination::Next(number),
                            "last" => Pagination::Last(number),
                            other => Pagination::Undefined(other.to_string(), number),
                        })
                        .collect::<Vec<_>>(),
                ),
                _ => None,
            },
        }
    }

    fn get_next_page(&self) -> Option<u64> {
        let result = self.get_pagination()?.into_iter().find(|it| match it {
            Pagination::Next(_) => true,
            _ => false,
        })?;

        if let Pagination::Next(value) = result {
            Some(value)
        } else {
            None
        }
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
    pub result: T,
    pub limits: Option<LimitRemainingReset>,
    pub owner: Option<String>,
    pub repository: Option<String>,
    pub next_page: Option<u64>,
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

#[derive(Debug)]
pub enum Pagination {
    First(u64),
    Previous(u64),
    Next(u64),
    Last(u64),
    Undefined(String, u64),
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
    author: GenericPerson,
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
    uploader: GenericPerson,
    content_type: String,
    state: String,
    size: u64,
    download_count: u64,
    created_at: String,
    updated_at: String,
    browser_download_url: String,
}

#[derive(Debug, Deserialize)]
pub struct GenericPerson {
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
