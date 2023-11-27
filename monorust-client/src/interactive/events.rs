use crossterm::event::{
    self,
    Event::Key,
    KeyCode::{Char, Down, Enter, Up},
    KeyModifiers,
};

use super::app::App;

pub fn update(app: &mut App) -> anyhow::Result<()> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Key(key) = event::read()? {
            app.text = format!("{key:?}");
            if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == Char('c') {
                app.should_quit = true;
            }
            else if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Up => app.previous_page(),
                    Down => app.next_page(),
                    Char('q') => app.should_quit = true,
                    Enter => match app.current_page {
                        super::pages::Page::Checkout => app.perform_checkout(),
                        super::pages::Page::List => app.list_checkouts(),
                        super::pages::Page::Clean => app.clean_dir(),
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
    }
    Ok(())
}
