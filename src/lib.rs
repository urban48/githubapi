use reqwest::Client;

use crate::helpers::{parse_json, HeaderMapExtensions};
use crate::types::*;

mod helpers;
mod macros;
mod types;

pub struct GitHubApi {
    username: String,
    password: String,
}

/// Implement basic functionality.
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
        let url = format!(
            "https://api.github.com/{}?per_page=100&page={}",
            method, page
        );

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
}

/// Implement rate limits.
impl GitHubApi {
    /// Gets rate limit information.
    pub fn get_rate_limit(&self) -> Response<RateLimitResponse> {
        let (text, limit_remaining_reset, _) = self.api_get_call("rate_limit", 1)?;

        Ok(GitHubApiResult {
            result: parse_json(&text)?,
            limits: limit_remaining_reset,
            owner: None,
            repository: None,
            next_page: None,
        })
    }
}

// Implement the license endpoint. No pagination.
make_single_page_api!(get_license, "license", LicenseResponse);

// Implement the tags endpoint, including an pagination iterator.
make_paginated_api!(get_tags, get_tags_page, "tags", TagPaginator, TagsResponse);

// Implement the releases endpoint, including an pagination iterator.
make_paginated_api!(
    get_releases,
    get_releases_page,
    "releases",
    ReleasePaginator,
    ReleasesResponse
);

// region Tests

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// endregion
