use ratatui::prelude::*;

use crate::interactive::App;

pub fn show<'a>(app: &App) -> Text<'a> {
    let checkouts: String = app
        .checkout_list
        .iter()
        .map(|checkout| format!("User: {}, Module: {}\n", checkout.user, checkout.module))
        .collect();

    format!(
        r#"
    Current checkouts (Press ENTER to refresh):

    {}
    "#,
        checkouts
    )
    .into()
}
