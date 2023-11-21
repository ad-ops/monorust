# Monorust
Playing around with prototype to work with monorepo in Rust.

## Client
CLI/TUI to interact with monorepo and server.

## Server
Example build/deploy server.

#### Random
<https://github.blog/2020-01-17-bring-your-monorepo-down-to-size-with-sparse-checkout/>

<https://github.com/libgit2/libgit2/issues/2263> :'(

1. Clone or Pull hard-coded repo
    - Assumption: Always empty (clone)
1. Get specific folder
    - Assumption: User already know which folder
1. Make changes locally
    - Normal git flow
1. Commit and push changes
1. Build release
1. Deploy release
