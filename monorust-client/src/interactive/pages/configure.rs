use ratatui::prelude::*;

use crate::interactive::App;

pub fn show<'a>(app: &App) -> Text<'a> {
    format!(
        r#"
    User
    {}

    Module
    {}

    Target Directory
    {}
    "#,
        app.user,
        app.module,
        app.target_dir.to_string_lossy()
    )
    .into()
}
