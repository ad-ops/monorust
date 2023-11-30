use monorust_models::Checkout;
use ratatui::prelude::*;

pub fn show<'a>(checkouts: &Vec<Checkout>) -> Text<'a> {
    let checkouts: String = checkouts
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
