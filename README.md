# This is a work in progress!
### _Please don't put it into production yet._

# What's there?
Not a whole lot, yet. The list of examples is pretty exhaustive.

# Envelope

## Result type
Every endpoint returns `Result<GitHubApiResult<T>, GitHubApiError>`.

### GitHubApiResult
```rust
pub struct GitHubApiResult<T> {
    pub result: T,
    pub raw_result: String,
    pub limits: Option<LimitRemainingReset>,
    pub owner: Option<String>,
    pub repository: Option<String>,
    pub next_page: Option<u64>,
}
```

### GitHubApiError
```rust
pub enum GitHubApiError {
    NotImplemented,
    JsonError((JsonError, String)),
    GitHubError((String, String)),
    ReqwestError(ReqwestError),
}
```

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
