# Bitbucket csproj version checker

A small CLI program for checking the version of a specific package across all repositories in a Bitbucket project. Written in Rust for no other reasons than I like the language.

Run like this:

```shell
cargo run -- --base-url="<base_url>" --project="<bitbucket_project_name>" --package="<package_name>" --token="<bitbucket_token>" --ignore-repo-prefix="<optional_ignore_prefix>"
```
