use ratatui::text::Text;

use super::App;

pub mod help;
pub mod configure;
pub mod checkout;
pub mod build;
pub mod deploy;
pub mod list;

pub fn page_output<'a>(app: &App) -> Text<'a> {
    match app.current_page {
        super::Page::Help => help::show(),
        super::Page::Configure => configure::show(app),
        super::Page::Checkout => checkout::show(app),
        super::Page::List => list::show(app),
        super::Page::Build => build::show(app),
        super::Page::Deploy => deploy::show(app),
    }
}