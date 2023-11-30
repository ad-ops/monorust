use ratatui::prelude::*;

use crate::interactive::App;

pub fn show<'a>(app: &'a App) -> Text<'a> {
    let help = vec![
        "\n".into(),
        "User".bold(),
        app.user.clone().yellow(),
        "\n".into(),
        "Module".bold(),
        app.module.clone().yellow(),
        "\n".into(),
        "Target Directory".bold(),
        app.target_dir
            .clone()
            .to_string_lossy()
            .to_string()
            .yellow(),
    ];

    help.into_iter()
        .map(|span| Line::from(span))
        .collect::<Vec<Line>>()
        .into()
}
