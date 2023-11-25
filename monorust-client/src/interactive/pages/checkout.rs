use ratatui::prelude::*;

use crate::interactive::App;

pub fn show<'a>(app: &App) -> Text<'a> {
    let do_check = "check ok!";
    let checkout_message = if let Some(message) = &app.checkout_message {
        message
    } else {
        ""
    };

    format!(
        r#"
    doing check: {do_check}
    Doing some checkout...

    Module: {}
    Target Directory: {:?}
    
    {}

    Press ENTER to do checkout...
    "#,
        app.module, app.target_dir, checkout_message
    )
    .into()
}
