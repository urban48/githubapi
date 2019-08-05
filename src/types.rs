use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize};
use serde_json::error::Error as JsonError;
use serde_json::Value;
use std::collections::HashMap;
use crate::helpers::ToJsonString;

#[macro_export]
macro_rules! impl_to_json_string {
    ($type_name:ty) => {
        impl ToJsonString for $type_name {
            fn to_json_string(&self) -> Result<String, JsonError> {
                serde_json::to_string(self)
            }
        }
    };
}

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
    JsonError((JsonError, String)),
    GitHubError((String, String)),
    ReqwestError(ReqwestError),
}

// endregion

// region Enums

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimitResponse {
    pub resources: RateLimitResources,

    // NOTE: The rate is deprecated by GitHub, and may disappear at any time.
    //       Use `resources.core` instead.
    pub rate: Option<LimitRemainingReset>,
}
impl_to_json_string!(RateLimitResponse);

#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimitResources {
    pub core: LimitRemainingReset,
    pub search: LimitRemainingReset,
    pub graphql: LimitRemainingReset,
    pub integration_manifest: LimitRemainingReset,
}
impl_to_json_string!(RateLimitResources);

#[derive(Debug, Serialize, Deserialize)]
pub struct LimitRemainingReset {
    pub limit: u64,
    pub remaining: u64,
    pub reset: u64,
}
impl_to_json_string!(LimitRemainingReset);

// endregion

// region TagsResponse

#[derive(Debug, Serialize, Deserialize)]
pub struct TagsResponse {
    pub name: String,
    pub zipball_url: String,
    pub tarball_url: String,
    pub commit: TagsCommit,
    pub node_id: String,

    #[serde(flatten)]
    pub uncaptured: HashMap<String, Value>,
}
impl_to_json_string!(TagsResponse);

#[derive(Debug, Serialize, Deserialize)]
pub struct TagsCommit {
    pub sha: String,
    pub url: String,

    #[serde(flatten)]
    pub uncaptured: HashMap<String, Value>,
}
impl_to_json_string!(TagsCommit);

// endregion

// region ReleasesResponse

#[derive(Debug, Serialize, Deserialize)]
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

    #[serde(flatten)]
    pub uncaptured: HashMap<String, Value>,
}
impl_to_json_string!(ReleasesResponse);

#[derive(Debug, Serialize, Deserialize)]
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

    #[serde(flatten)]
    pub uncaptured: HashMap<String, Value>,
}
impl_to_json_string!(ReleasesAsset);

#[derive(Debug, Serialize, Deserialize)]
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

    #[serde(flatten)]
    pub uncaptured: HashMap<String, Value>,
}
impl_to_json_string!(GenericPerson);

// endregion

// region LicenseResponse

#[derive(Debug, Serialize, Deserialize)]
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

    #[serde(flatten)]
    pub uncaptured: HashMap<String, Value>,
}
impl_to_json_string!(LicenseResponse);

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseLinks {
    #[serde(rename(deserialize = "self"))]
    pub self_link: String,
    pub git: String,
    pub html: String,

    #[serde(flatten)]
    pub uncaptured: HashMap<String, Value>,
}
impl_to_json_string!(LicenseLinks);

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseLicense {
    pub key: String,
    pub name: String,
    pub spdx_id: String,
    pub url: String,
    pub node_id: String,

    #[serde(flatten)]
    pub uncaptured: HashMap<String, Value>,
}
impl_to_json_string!(LicenseLicense);

// endregion


