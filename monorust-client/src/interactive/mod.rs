pub mod app;
pub mod events;
pub mod pages;
pub mod ui;

use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::{io::stdout, path::PathBuf};

use app::App;

use events::update;
use ui::ui;

#[derive(PartialEq)]
pub enum Pane {
    Menu,
    Output,
    Input,
}

impl Pane {
    pub fn next_pane(&self) -> Self {
        match self {
            Pane::Menu => Pane::Output,
            Pane::Output => Pane::Input,
            Pane::Input => Pane::Menu,
        }
    }
}

pub fn run_interactive(module: Option<String>, target_dir: Option<PathBuf>) -> Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::new(
        "myuser",
        &module.unwrap_or("module1".to_string()),
        target_dir.unwrap_or("target".into()),
    );

    loop {
        // draw
        terminal.draw(|f| {
            ui(&app, f);
        })?;

        // update state
        update(&mut app)?;

        // take action
        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    println!("Ok, bye!");
    Ok(())
}
