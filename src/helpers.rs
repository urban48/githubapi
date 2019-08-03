// region Helpers
use crate::types::{GitHubApiError, LimitRemainingReset};
use crate::Pagination;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;

pub trait HeaderMapExtensions {
    fn get_as_u64(&self, key: &str) -> Option<u64>;

    fn get_rate_limits(&self) -> Option<LimitRemainingReset>;
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

    fn get_pagination(&self) -> Option<Vec<Pagination>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r#"<.*?[\?&]page=(\d+).*?>; rel="(.*?)""#).unwrap();
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

pub fn parse_json<'a, T>(text: &'a str) -> Result<T, GitHubApiError>
where
    T: Deserialize<'a>,
{
    match serde_json::from_str(&text) {
        Ok(value) => Ok(value),
        Err(error) => Err(GitHubApiError::JsonError(error)),
    }
}

// endregion
