use crossterm::{
    event::{
        self,
        Event::Key,
        KeyCode::Char,
    },
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::{io::stdout, fmt::Display};

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
    current_page: Page,
    should_quit: bool,
}

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App {
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
    Ok(())
}

fn update(app: &mut App) -> anyhow::Result<()> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('q') => app.should_quit = true,
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
        Paragraph::new(format!("Text: {}", "Hello World!"))
            .block(Block::default().title(app.current_page.to_string()).borders(Borders::ALL)),
        f.size(),
    );
}