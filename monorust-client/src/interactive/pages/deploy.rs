use ratatui::prelude::*;

use crate::interactive::App;

pub fn show<'a>(app: &App) -> Text<'a> {
    format!(r#"
    Deploying stuff...
    "#).into()
}