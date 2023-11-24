use ratatui::prelude::*;

use crate::interactive::App;

pub fn show<'a>(app: &App) -> Text<'a> {
    let do_check = "check ok!";

    format!(r#"
    doing check: {do_check}
    Doing some checkout...

    Module: {}
    Target Directory: {}
    Press ENTER to do checkout...
    "#, app.user, app.module).into()
}