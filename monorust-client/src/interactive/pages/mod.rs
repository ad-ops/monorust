use std::fmt::Display;

use ratatui::text::Text;

use super::App;

pub mod build;
pub mod checkout;
pub mod configure;
pub mod deploy;
pub mod help;
pub mod list;

pub enum Page {
    Help,
    Configure,
    Checkout,
    List,
    Build,
    Deploy,
}

impl Page {
    // Could be improved with strum-crate
    pub fn list_all() -> Vec<&'static str> {
        vec!["Help", "Configure", "Checkout", "List", "Build", "Deploy"]
    }

    pub fn previous_page(&self) -> Self {
        match self {
            Page::Help => Page::Help,
            Page::Configure => Page::Help,
            Page::Checkout => Page::Configure,
            Page::List => Page::Checkout,
            Page::Build => Page::List,
            Page::Deploy => Page::Build,
        }
    }

    pub fn next_page(&self) -> Self {
        match self {
            Page::Help => Page::Configure,
            Page::Configure => Page::Checkout,
            Page::Checkout => Page::List,
            Page::List => Page::Build,
            Page::Build => Page::Deploy,
            Page::Deploy => Page::Deploy,
        }
    }
}

impl Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Page::Help => "Help",
            Page::Configure => "Configure",
            Page::Checkout => "Checkout",
            Page::List => "List",
            Page::Build => "Build",
            Page::Deploy => "Deploy",
        };
        write!(f, "{text}")
    }
}

pub fn page_output<'a>(app: &App) -> Text<'a> {
    match app.current_page {
        Page::Help => help::show(),
        Page::Configure => configure::show(app),
        Page::Checkout => checkout::show(app),
        Page::List => list::show(app),
        Page::Build => build::show(app),
        Page::Deploy => deploy::show(app),
    }
}
