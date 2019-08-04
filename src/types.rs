use reqwest::Error as ReqwestError;
use serde::Deserialize;
use serde_json::error::Error as JsonError;

// region Envelopes

pub type Response<T> = Result<GitHubApiResult<T>, GitHubApiError>;

#[derive(Debug)]
pub struct GitHubApiResult<T> {
    pub result: T,
    pub raw_result: String,
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

    // NOTE: The rate is deprecated by GitHub, and may disappear at any time.
    //       Use `resources.core` instead.
    pub rate: Option<LimitRemainingReset>,
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
    pub name: String,
    pub zipball_url: String,
    pub tarball_url: String,
    pub commit: TagsCommit,
    pub node_id: String,
}

#[derive(Debug, Deserialize)]
pub struct TagsCommit {
    pub sha: String,
    pub url: String,
}

// endregion

// region ReleasesResponse

#[derive(Debug, Deserialize)]
pub struct ReleasesResponse {
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub html_url: String,
    pub id: u64,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub author: GenericPerson,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub assets: Vec<ReleasesAsset>,
    pub tarball_url: String,
    pub zipball_url: String,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct ReleasesAsset {
    pub url: String,
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub label: Option<String>,
    pub uploader: GenericPerson,
    pub content_type: String,
    pub state: String,
    pub size: u64,
    pub download_count: u64,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}

#[derive(Debug, Deserialize)]
pub struct GenericPerson {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub r#type: String,
    pub site_admin: bool,
}

// endregion

// region LicenseResponse

#[derive(Debug, Deserialize)]
pub struct LicenseResponse {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: u64,
    pub url: String,
    pub html_url: String,
    pub git_url: String,
    pub download_url: String,
    pub r#type: String,
    pub content: String,
    pub encoding: String,
    #[serde(rename(deserialize = "_links"))]
    pub links: LicenseLinks,
    pub license: LicenseLicense,
}

#[derive(Debug, Deserialize)]
pub struct LicenseLinks {
    #[serde(rename(deserialize = "self"))]
    pub self_link: String,
    pub git: String,
    pub html: String,
}

#[derive(Debug, Deserialize)]
pub struct LicenseLicense {
    pub key: String,
    pub name: String,
    pub spdx_id: String,
    pub url: String,
    pub node_id: String,
}

// endregion
