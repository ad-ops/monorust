use crossterm::event::{
    self,
    Event::Key,
    KeyCode::{Backspace, Char, Down, Enter, Tab, Up},
    KeyModifiers,
};

use super::{app::App, Pane};

pub fn update(app: &mut App) -> anyhow::Result<()> {
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
                        Enter => match app.current_page {
                            super::pages::Page::Checkout => app.perform_checkout(),
                            super::pages::Page::List => app.list_checkouts(),
                            _ => {}
                        },
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
