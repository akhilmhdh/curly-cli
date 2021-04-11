mod app;
mod input;
mod ui;

use crate::input::{Events, Key};

use app::App;
use crossterm::terminal::enable_raw_mode;
use std::{error::Error, io};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), Box<dyn Error>> {
    // raw mode: https://docs.rs/crossterm/0.3.0/crossterm/raw/index.html
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let app = App::new();
    let events = Events::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            app.render(f, size);
        })?;
        match events.next()? {
            Key::Esc => {
                break;
            }
            _ => println!("Key pressed"),
        }
    }
    Ok(())
}
