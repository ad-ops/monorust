use ratatui::{prelude::*, widgets::*};

use super::{
    app::App,
    pages::{self, Page},
};

pub fn ui(app: &App, f: &mut Frame<'_>) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(15), Constraint::Percentage(90)])
        .split(f.size());

    // Menu
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
    f.render_widget(
        Paragraph::new(Text::from(menu_options))
            .block(Block::new().borders(Borders::ALL).title("Menu")),
        layout[0],
    );

    // Output
    f.render_widget(
        Paragraph::new(pages::page_output(app))
            .wrap(Wrap { trim: true })
            .scroll((1, 0))
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title(app.current_page.to_string()),
            ),
        layout[1],
    );
}
