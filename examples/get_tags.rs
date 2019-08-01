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
    gh: &'a GitHubApi,
    owner: String,
    repository: String,
    page: Option<ApiResponse<Vec<TagsResponse>>>,
}

impl<'a> TagIterator<'a> {
    fn new(gh: &'a GitHubApi, owner: &str, repository: &str) -> Self {
        Self {
            gh,
            owner: owner.to_string(),
            repository: repository.to_string(),
            page: None,
        }
    }
}

impl<'a> Iterator for TagIterator<'a> {
    type Item = ApiResponse<Vec<TagsResponse>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.page.is_none() {
            self.page = Some(self.gh.get_tags(&self.owner, &self.repository).unwrap());
            self.page.clone()
        } else if self.page.clone()?.next_page.is_some() {
            self.page = Some(self.gh.get_tags_next(&self.page.clone()?).unwrap());
            self.page.clone()
        } else {
            None
        }
    }
}
