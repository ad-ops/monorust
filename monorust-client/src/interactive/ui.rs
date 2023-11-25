use ratatui::{prelude::*, widgets::*};

use super::{
    app::App,
    pages::{self, Page},
    Pane,
};

pub fn ui(app: &App, f: &mut Frame<'_>) {
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
            .scroll((1, 0))
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
