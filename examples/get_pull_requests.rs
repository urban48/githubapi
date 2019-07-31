use std::env;
use githubapi::GitHubApi;

fn main() {
    let username = env::var("GH_USER").expect("GH_USER not defined.");
    let password = env::var("GH_PASS").expect("GH_PASS not defined.");

    let gh = GitHubApi::new(&username, &password);
    let pulls = gh.get_pull_requests("sous-chefs", "postgresql");

    println!("{:#?}", pulls);
}
