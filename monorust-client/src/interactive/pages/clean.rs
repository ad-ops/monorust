use ratatui::prelude::*;

use crate::interactive::App;

pub fn show<'a>(app: &App) -> Text<'a> {
    let clean_message = "Delete everything under target dir? Press ENTER";

    format!(
        r#"
    Target Directory
    {:?}

    {}
    "#,
        app.target_dir, clean_message
    )
    .into()
}
