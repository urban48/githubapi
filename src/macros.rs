#[macro_export]
macro_rules! make_paginated_api {
    ($function_name:ident, $endpoint:expr, $iterator_name:ident, $return_type:ty) => {
        impl GitHubApi {
            pub fn $function_name(
                &self,
                owner: &str,
                repository: &str,
                page: u64,
            ) -> Response<Vec<$return_type>> {
                let method = format!("repos/{}/{}/{}", owner, repository, $endpoint);
                let (text, limit_remaining_reset, next_page) = self.api_get_call(&method, page)?;

                Ok(ApiResponse {
                    result: parse_json(&text)?,
                    limits: limit_remaining_reset,
                    owner: Some(owner.to_string()),
                    repository: Some(repository.to_string()),
                    next_page,
                })
            }
        }

        pub struct $iterator_name<'a> {
            github_api: &'a GitHubApi,
            owner: String,
            repository: String,
            next_page: Option<u64>,
        }

        impl<'a> $iterator_name<'a> {
            pub fn new(github_api: &'a GitHubApi, owner: &str, repository: &str) -> Self {
                Self {
                    github_api,
                    owner: owner.to_string(),
                    repository: repository.to_string(),
                    next_page: Some(1),
                }
            }
        }

        impl<'a> Iterator for $iterator_name<'a> {
            type Item = ApiResponse<Vec<$return_type>>;

            fn next(&mut self) -> Option<Self::Item> {
                match self.next_page {
                    Some(page_number) => {
                        let page = self
                            .github_api
                            .$function_name(&self.owner, &self.repository, page_number)
                            .unwrap();

                        self.next_page = page.next_page.clone();
                        Some(page)
                    }
                    None => None,
                }
            }
        }
    };
}
