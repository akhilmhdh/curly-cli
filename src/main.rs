mod input;

// use crate::input::{Events, Key};

// use crossterm::terminal::enable_raw_mode;
use std::error::Error;
use std::io;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders};
use tui::Terminal;

fn main() -> Result<(), Box<dyn Error>> {
    // raw mode: https://docs.rs/crossterm/0.3.0/crossterm/raw/index.html
    // enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    loop {
        terminal.draw(|f| {
            let size = f.size();

            let main_layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(size);

            let block = Block::default().title("Body").borders(Borders::ALL);
            f.render_widget(block, main_layout[1]);

            let top_layout = Layout::default()
                .direction(Direction::Horizontal)
                .horizontal_margin(1)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(main_layout[0]);

            let block = Block::default().title("Type").borders(Borders::ALL);
            f.render_widget(block, top_layout[0]);

            let block = Block::default().title("URL").borders(Borders::ALL);
            f.render_widget(block, top_layout[1]);
        })?;
    }
    Ok(())
}
