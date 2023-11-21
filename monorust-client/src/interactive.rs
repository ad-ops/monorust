use crossterm::{
    event::{self, Event::Key, KeyCode::Char, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::{io::stdout, fmt::Display};
use anyhow::Result;
use crate::server;

enum Page {
    Init,
    Hello,
}

impl Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Page::Init => "Init",
            Page::Hello => "Hello",
        };
        write!(f, "{text}")
    }
}

struct App {
    text: String,
    current_page: Page,
    should_quit: bool,
}

pub fn run_interactive() -> Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App {
        text: "placeholder".into(),
        current_page: Page::Init,
        should_quit: false,
    };

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

fn update(app: &mut App) -> anyhow::Result<()> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Key(key) = event::read()? {
            // app.text = format!("{key:?}");
            if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == Char('c') {
                app.should_quit = true;
            } else if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('q') => app.should_quit = true,
                    Char('r') => app.text = server::say_hello()?,
                    Char(_c) => app.current_page = Page::Hello,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn ui(app: &App, f: &mut Frame<'_>) {
    f.render_widget(
        Paragraph::new(format!("Text: {}", app.text))
            .block(
                Block::default()
                    .title(app.current_page.to_string())
                    .borders(Borders::ALL),
            ),
        f.size(),
    );
}
