use githubapi::{GitHubApi, TagPaginator};
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

    for page in TagPaginator::new(&gh, "sous-chefs", "postgresql") {
        println!("{:#?}", page);
    }
}
