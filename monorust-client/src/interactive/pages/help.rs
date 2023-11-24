use ratatui::prelude::*;

pub fn show<'a>() -> Text<'a> {
    r#"
    This TUI is a prototype on how a tool can work with a monorepo in git 
    and handle complex commands and interacting with a server.

    Usage
    You can exit using CTRL+c
    Change PANE using TAB
    Change PAGE with arrow keys when on MENU

    0. Configure: Set which module to work on and where to clone to
    1. Checkout: Runs git commands to sparse clone/checkout individual folders in monorepo
    2. *Change: Done outside this tool with normal git-commands
    3. Build: Create a compile package based on dependencies and run compilation
    4. Deploy: Deploy a built compilation package
    "#
    .into()
}
