use githubapi::{ApiResponse, GitHubApi, TagsResponse};
use std::env;

/// To run this example, you must first set two environment variables.
/// ```bash
/// export GH_USER="SandyClaws"
/// export GH_PASS="ThisIsHalloween"
///
/// cargo run --example get_pull_requests
/// ```
fn main() {
    let username = env::var("GH_USER").expect("GH_USER not defined.");
    let password = env::var("GH_PASS").expect("GH_PASS not defined.");

    let gh = GitHubApi::new(&username, &password);

    for page in TagIterator::new(&gh, "sous-chefs", "postgresql") {
        println!("{:#?}", page);
    }
}

// The code below shows how we can use the iterator pattern.

struct TagIterator<'a> {
    github_api: &'a GitHubApi,
    owner: String,
    repository: String,
    next_page: Option<u64>,
}

impl<'a> TagIterator<'a> {
    fn new(github_api: &'a GitHubApi, owner: &str, repository: &str) -> Self {
        Self {
            github_api,
            owner: owner.to_string(),
            repository: repository.to_string(),
            next_page: Some(1),
        }
    }
}

impl<'a> Iterator for TagIterator<'a> {
    type Item = ApiResponse<Vec<TagsResponse>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_page {
            Some(page_number) => {
                let page = self
                    .github_api
                    .get_tags_page(&self.owner, &self.repository, page_number)
                    .unwrap();
                self.next_page = page.next_page.clone();
                Some(page)
            }
            None => None,
        }
    }
}
