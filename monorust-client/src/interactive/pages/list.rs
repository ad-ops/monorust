use ratatui::prelude::*;

use crate::interactive::App;

pub fn show<'a>(_app: &App) -> Text<'a> {
    format!(
        r#"
    listing stuff...
    "#
    )
    .into()
}