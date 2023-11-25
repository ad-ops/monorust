# Monorust
Playing around with prototype to work with monorepo in Rust.

The idea is to use [sparse-checkout](https://github.blog/2020-01-17-bring-your-monorepo-down-to-size-with-sparse-checkout/) to reduce the number of files when cloning.

if [libgit2 issue:2263](https://github.com/libgit2/libgit2/issues/2263) could be fixed or the monorepo is small enough it would be advantageous to use [git2](https://crates.io/crates/git2) instead of commands to interact with git.

The main part is a client on a users machine that can wrap/issue all commands needed and communicate with a server that holds state.

### Client
CLI/TUI to interact with monorepo and server.

For fun trying out [ratatui](https://ratatui.rs) and how it is to build this.

### Server
Axum server with APIs storing state in SQLite.

## Run
```sh
# Clone this repo from monorepo-example
cargo run --bin monorust-client -- checkout --module "module1" --target-dir "./target"
# Remove cloned dir
cargo run --bin monorust-client -- clean --target-dir "./target"
# Run as TUI
cargo run --bin monorust-client -- interactive --module "module1" --target-dir "./target"


# Use zellij to run everything together
zellij --layout dev_layout.kdl
```

## Thoughts
- Server to run as a Shuttle service with Turso.
- Token to make sure only registered clients can access.
- Web client as alternative to CLI. 


