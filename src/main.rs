use std::io;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ]
                .as_ref(),
            )
            .split(f.size());
        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        let block = Block::default().title("Block 2").borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
        let block = Block::default().title("Block 3").borders(Borders::ALL);
        f.render_widget(block, chunks[2]);
    })?;
    Ok(())
}
