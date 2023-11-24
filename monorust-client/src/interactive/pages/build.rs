use ratatui::prelude::*;

use crate::interactive::App;

pub fn show<'a>(app: &App) -> Text<'a> {
    format!(r#"
    building stuff...
    "#).into()
}