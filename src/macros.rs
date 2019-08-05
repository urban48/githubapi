// make_single_page_api!(
//     // You make this name up. A good idea is to call it get_SOMETHING,
//     // where SOMETHING is the name of what you're querying.
//     // It'll be added to GitHubApi as a function.
//     get_license,
//
//     // This is the last word in the url you're querying.
//     // In this case it's the last category of "/repos/:owner/:repo/license".
//     "license",
//
//     // This is the name of the type your Paginator produces.
//     // If you're getting users, then the correct thing to name this would be UsersResponse.
//     // Responses are currently located in types.rs.
//     LicenseResponse
// );

#[macro_export]
macro_rules! make_single_page_api {
    ($function_name:ident, $endpoint:expr, $return_type:ty) => {
        impl GitHubApi {
            /// Gets the page.
            pub fn $function_name(&self, owner: &str, repository: &str) -> Response<$return_type> {
                let method = format!("repos/{}/{}/{}", owner, repository, $endpoint);
                let (text, limit_remaining_reset, next_page) =
                    self.api_get_call(&method, 1, 100)?;

                Ok(GitHubApiResult {
                    result: parse_json(&text)?,
                    raw_result: text,
                    limits: limit_remaining_reset,
                    owner: Some(owner.to_string()),
                    repository: Some(repository.to_string()),
                    next_page,
                })
            }
        }
    };
}

// make_paginated_api!(
//     // You make this name up. A good idea is to call it get_SOMETHINGS,
//     // where SOMETHINGS is the name of what you're querying, pluralized.
//     // It'll be added to GitHubApi as a function.
//     get_tags,
//
//     // You make this name up. A good idea is to call it get_SOMETHING_page,
//     // where SOMETHING is the name of what you're querying.
//     // It'll be added to GitHubApi as a function.
//     get_tags_page,
//
//     // This is the last word in the url you're querying.
//     // In this case it's the last category of "/repos/:owner/:repo/git/tags".
//     "tags",
//
//     // You make this name up too. It's the name of the paginator iterator
//     // that this macro creates.
//     TagPaginator,
//
//     // This is the name of the type your Paginator produces.
//     // If you're getting users, then the correct thing to name this would be UsersResponse.
//     // Responses are currently located in types.rs.
//     TagsResponse
// );

#[macro_export]
macro_rules! make_paginated_api {
    ($paginator_function_name:ident, $page_function_name:ident, $endpoint:expr, $paginator_name:ident, $return_type:ty) => {
        impl GitHubApi {
            /// Gets a single page.
            pub fn $page_function_name(
                &self,
                owner: &str,
                repository: &str,
                page: u64,
            ) -> Response<Vec<$return_type>> {
                let method = format!("repos/{}/{}/{}", owner, repository, $endpoint);
                let (text, limit_remaining_reset, next_page) =
                    self.api_get_call(&method, page, 100)?;

                Ok(GitHubApiResult {
                    result: parse_json(&text)?,
                    raw_result: text,
                    limits: limit_remaining_reset,
                    owner: Some(owner.to_string()),
                    repository: Some(repository.to_string()),
                    next_page,
                })
            }

            /// Creates a paginator.
            pub fn $paginator_function_name(
                &self,
                owner: &str,
                repository: &str,
            ) -> $paginator_name {
                $paginator_name::new(&self, owner, repository)
            }
        }

        pub struct $paginator_name<'a> {
            github_api: &'a GitHubApi,
            owner: String,
            repository: String,
            next_page: Option<u64>,
        }

        impl<'a> $paginator_name<'a> {
            pub fn new(github_api: &'a GitHubApi, owner: &str, repository: &str) -> Self {
                Self {
                    github_api,
                    owner: owner.to_string(),
                    repository: repository.to_string(),
                    next_page: Some(1),
                }
            }

            pub fn has_items(&self) -> Result<bool, GitHubApiError> {
                let method = format!("repos/{}/{}/{}", self.owner, self.repository, $endpoint);
                let (text, _, _) = self.github_api.api_get_call(&method, 1, 1)?;
                let response: Vec<$return_type> = parse_json(&text)?;
                Ok(!response.is_empty())
            }
        }

        impl<'a> Iterator for $paginator_name<'a> {
            type Item = GitHubApiResult<Vec<$return_type>>;

            /// Gets the next page.
            fn next(&mut self) -> Option<Self::Item> {
                match self.next_page {
                    Some(page_number) => {
                        let requested_page = self.github_api.$page_function_name(
                            &self.owner,
                            &self.repository,
                            page_number,
                        );

                        match requested_page {
                            Ok(page) => {
                                self.next_page = page.next_page.clone();
                                Some(page)
                            }
                            Err(error) => {
                                eprintln!("Error: {:#?}", error);
                                None
                            }
                        }
                    }
                    None => None,
                }
            }
        }
    };
}
