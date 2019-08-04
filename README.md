# This is a work in progress!
### _Please don't put it into production yet._

# What's there?
Not a whole lot, yet. The list of examples is pretty exhaustive.

# Examples

## Get rate limit
```rust
let gh = GitHubApi::new(&username, &password);
let result = gh.get_rate_limit();
println!("{:#?}", result);
```

## Get license
```rust
let gh = GitHubApi::new(&username, &password);
let license = gh.get_license("segfaultsourcery", "githubapi");
println!("{:#?}", license);
```

## Get releases
```rust
let gh = GitHubApi::new(&username, &password);
for page in gh.get_releases("segfaultsourcery", "githubapi") {
    println!("{:#?}", page);
}
```

## Get tags
```rust
let gh = GitHubApi::new(&username, &password);
for page in gh.get_tags("segfaultsourcery", "githubapi") {
    println!("{:#?}", page);
}
```
