use githubapi::GitHubApi;
use std::env;

/// To run this example, you must first set two environment variables.
/// ```bash
/// export GH_USER="SandyClaws"
/// export GH_PASS="ThisIsHalloween"
///
/// cargo run --example get_releases_or_tags
/// ```
fn main() {
    let username = env::var("GH_USER").expect("GH_USER not defined.");
    let password = env::var("GH_PASS").expect("GH_PASS not defined.");

    let gh = GitHubApi::new(&username, &password);

    let owner = "rabbitmq";
    let repository = "rabbitmq-server";

    let releases_paginator = gh.get_releases(owner, repository);

    match releases_paginator.has_items() {
        Ok(true) => {
            println!("There were releases. Get them.");

            // for page in releases_paginator {
            //     println!("{:#?}", page);
            // }
        }

        Ok(false) => {
            println!("There were no releases. Get the tags instead.");

            // for page in gh.get_tags(owner, repository) {
            //     println!("{:#?}", page);
            // }
        }

        Err(error) => {
            println!("There was a problem: {:#?}.", error);
        }
    }
}
