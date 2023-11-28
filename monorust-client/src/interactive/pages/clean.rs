use ratatui::prelude::*;

use crate::interactive::App;

pub fn show<'a>(app: &App) -> Text<'a> {
    format!(
        r#"
    Target Directory
    {:?}

    {}

    Delete everything under target dir? Press ENTER
    "#,
        app.target_dir.join("monorust"),
        app.cleaned_dir_message
    )
    .into()
}
