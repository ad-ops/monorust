# Monorust
Playing around with prototype to work with monorepo in Rust.

if [libgit2 issue:2263](https://github.com/libgit2/libgit2/issues/2263) could be fixed or the monorepo is small enough it would be advantageous to use [git2](https://crates.io/crates/git2) instead of commands to interact with git.

## Client
CLI/TUI to interact with monorepo and server.

#### Run
```sh
# Clone this repo from monorepo-example
cargo run --bin monorust-client -- checkout --module "module1" --target-dir "./target"
# Remove cloned dir
cargo run --bin monorust-client -- clean --target-dir "./target"
# Run as TUI
cargo run --bin monorust-client -- interactive
```

## Server
Example build/deploy server.

#### Random
<https://github.blog/2020-01-17-bring-your-monorepo-down-to-size-with-sparse-checkout/>



1. Clone or Pull hard-coded repo
    - Assumption: Always empty (clone)
1. Get specific folder
    - Assumption: User already know which folder
1. Make changes locally
    - Normal git flow
1. Commit and push changes
1. Build release
1. Deploy release
