use githubapi::{GitHubApi, ReleasePaginator};
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

    // This is the "primitive" pagination approach. It's actually very simple.
    // The alternative is to use an iterator, which you'll find in `get_tags.rs`.

    for page in ReleasePaginator::new(&gh, "sous-chefs", "postgresql") {
        println!("{:#?}", page);
    }
}
