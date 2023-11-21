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
use clap::Parser;
use std::{io::stdout, fmt::Display, thread, time::Duration, path::PathBuf};

mod git;

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

/// Working with monorepo
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
    struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let x = git::clone(&PathBuf::from("/Users/mork/ws/rust/monorust/target"), "module1")?;
    println!("response: {x}");

//     thread::sleep(Duration::from_secs(3));

//     enable_raw_mode()?;
//     stdout().execute(EnterAlternateScreen)?;
//     let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
//     let mut app = App {
//         text: "Hello World!".into(),
//         current_page: Page::Init,
//         should_quit: false,
//     };

//     loop {
//         // draw
//         terminal.draw(|f| {
//             ui(&app, f);
//         })?;

//         // update state
//         update(&mut app)?;

//         // take action
//         if app.should_quit {
//             break;
//         }
//     }

//     disable_raw_mode()?;
//     stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

// fn update(app: &mut App) -> anyhow::Result<()> {
//     if event::poll(std::time::Duration::from_millis(50))? {
//         if let Key(key) = event::read()? {
//             if key.kind == event::KeyEventKind::Press {
//                 match key.code {
//                     Char('q') => app.should_quit = true,
//                     Char('e') => {
//                         let test = git::clone(&PathBuf::from("/Users/mork/ws/rust/monorust/target"), "module1")?;
//                         app.text = test;
//                     },
//                     Char(_c) => app.current_page = Page::Hello,
//                     _ => {}
//                 }
//             }
//         }
//     }
//     Ok(())
// }

// fn ui(app: &App, f: &mut Frame<'_>) {
//     f.render_widget(
//         Paragraph::new(format!("Text: {}", app.text))
//             .block(Block::default().title(app.current_page.to_string()).borders(Borders::ALL)),
//         f.size(),
//     );
// }