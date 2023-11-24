pub mod pages;

use anyhow::Result;
use crossterm::{
    event::{
        self,
        Event::Key,
        KeyCode::{Backspace, Char, Down, Tab, Up},
        KeyModifiers,
    },
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::{io::stdout, path::PathBuf};

use pages::Page;

#[derive(PartialEq)]
enum Pane {
    Menu,
    Output,
    Input,
}

impl Pane {
    fn next_pane(&self) -> Self {
        match self {
            Pane::Menu => Pane::Output,
            Pane::Output => Pane::Input,
            Pane::Input => Pane::Menu,
        }
    }
}

pub struct App {
    text: String,
    current_page: Page,
    current_pane: Pane,
    should_quit: bool,
    user: String,
    module: String,
    target_dir: PathBuf,
}
impl App {
    pub fn new(user: &str, module: &str, target_dir: PathBuf) -> Self {
        Self {
            text: String::new(),
            current_page: Page::Help,
            current_pane: Pane::Menu,
            should_quit: false,
            user: user.to_string(),
            module: module.to_string(),
            target_dir,
        }
    }

    pub fn perform_checkout(&self) -> Result<String> {
        Ok("Checkout done!".to_string())
    }

    pub fn previous_page(&mut self) {
        self.current_page = self.current_page.previous_page()
    }

    pub fn next_page(&mut self) {
        self.current_page = self.current_page.next_page()
    }

    pub fn next_pane(&mut self) {
        self.current_pane = self.current_pane.next_pane()
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

fn update(app: &mut App) -> anyhow::Result<()> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Key(key) = event::read()? {
            // app.text = format!("{key:?}");
            if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == Char('c') {
                app.should_quit = true;
            } else if key.kind == event::KeyEventKind::Press {
                match app.current_pane {
                    Pane::Menu => {
                        match key.code {
                            Up => app.previous_page(),
                            Down => app.next_page(),
                            Tab => app.next_pane(),
                            Char('q') => app.should_quit = true,
                            // Char('r') => app.text = server::say_hello()?,
                            _ => {}
                        }
                    }
                    Pane::Output => match key.code {
                        Tab => app.next_pane(),
                        _ => {}
                    },
                    Pane::Input => match key.code {
                        Tab => app.next_pane(),
                        Backspace => {
                            let _ = app.text.pop();
                        }
                        Char(c) => app.text.push(c),
                        _ => {}
                    },
                }
            }
        }
    }
    Ok(())
}

fn ui(app: &App, f: &mut Frame<'_>) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(15), Constraint::Percentage(90)])
        .split(f.size());
    let sub_layout = Layout::default()
        .direction(Direction::Vertical)
        // TODO: How to make the second pane min 1?
        .constraints([Constraint::Percentage(90), Constraint::Min(1)])
        .split(layout[1]);

    let menu_options: Vec<Line> = Page::list_all()
        .iter()
        .map(|o| {
            if o.to_string() == app.current_page.to_string() {
                Line::from(format!("-> {o}").bold().yellow())
            } else {
                Line::from(format!("   {o}").bold())
            }
        })
        .collect();

    // Menu
    f.render_widget(
        Paragraph::new(Text::from(menu_options)).block(
            Block::new()
                .borders(Borders::ALL)
                .border_style(match app.current_pane {
                    Pane::Menu => Style::new().yellow(),
                    _ => Style::new(),
                })
                .title("Menu"),
        ),
        layout[0],
    );

    // Output
    f.render_widget(
        Paragraph::new(pages::page_output(app))
            .wrap(Wrap { trim: true })
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_style(match app.current_pane {
                        Pane::Output => Style::new().yellow(),
                        _ => Style::new(),
                    })
                    .title(app.current_page.to_string()),
            ),
        sub_layout[0],
    );

    // Input
    f.render_widget(
        Paragraph::new(format!("{}", app.text)).block(
            Block::new()
                .borders(Borders::ALL)
                .border_style(match app.current_pane {
                    Pane::Input => Style::new().yellow(),
                    _ => Style::new(),
                })
                .title("Input"),
        ),
        sub_layout[1],
    );
}
