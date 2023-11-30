use std::fmt::Display;

use monorust_models::Checkout;
use ratatui::text::Text;

use super::App;

pub mod build;
pub mod checkout;
pub mod clean;
pub mod configure;
pub mod deploy;
pub mod help;
pub mod list;

pub enum Page {
    Help,
    Configure,
    Checkout,
    List(Vec<Checkout>),
    Build,
    Deploy,
    Clean,
}

impl Page {
    // Could be improved with strum-crate
    pub fn list_all() -> Vec<&'static str> {
        vec![
            "Help",
            "Configure",
            "Checkout",
            "List",
            "Build",
            "Deploy",
            "Clean",
        ]
    }

    pub fn previous_page(&self) -> Self {
        match self {
            Page::Help => Page::Help,
            Page::Configure => Page::Help,
            Page::Checkout => Page::Configure,
            Page::List(_) => Page::Checkout,
            Page::Build => Page::List(Vec::new()),
            Page::Deploy => Page::Build,
            Page::Clean => Page::Deploy,
        }
    }

    pub fn next_page(&self) -> Self {
        match self {
            Page::Help => Page::Configure,
            Page::Configure => Page::Checkout,
            Page::Checkout => Page::List(Vec::new()),
            Page::List(_) => Page::Build,
            Page::Build => Page::Deploy,
            Page::Deploy => Page::Clean,
            Page::Clean => Page::Clean,
        }
    }
}

impl Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Page::Help => "Help",
            Page::Configure => "Configure",
            Page::Checkout => "Checkout",
            Page::List(_) => "List",
            Page::Build => "Build",
            Page::Deploy => "Deploy",
            Page::Clean => "Clean",
        };
        write!(f, "{text}")
    }
}

pub fn page_output<'a>(app: &App) -> Text<'a> {
    match &app.current_page {
        Page::Help => help::show(),
        Page::Configure => configure::show(app),
        Page::Checkout => checkout::show(app),
        Page::List(checkouts) => list::show(&checkouts),
        Page::Build => build::show(app),
        Page::Deploy => deploy::show(app),
        Page::Clean => clean::show(app),
    }
}
