use ::tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
    Frame,
};
use std::error::Error;

/**
 * Home Screen
 * Three sections.
 * 1. Req Type --- 2. URL Bar ---
 * 3. -----------Body------------
 */
pub fn draw<B: Backend>(f: &mut Frame<B>, size: Rect) -> Result<(), Box<dyn Error>> {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(1)
        .constraints([Constraint::Length(3), Constraint::Percentage(80)].as_ref())
        .split(size);

    let block = Block::default().title("Body").borders(Borders::ALL);
    f.render_widget(block, main_layout[1]);

    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(main_layout[0]);

    let block = Block::default().title("Type").borders(Borders::ALL);
    f.render_widget(block, top_layout[0]);

    let block = Block::default().title("URL").borders(Borders::ALL);
    f.render_widget(block, top_layout[1]);
    Ok(())
}
