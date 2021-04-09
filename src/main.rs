mod event;
use crossterm::terminal::enable_raw_mode;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let events = event::Events::new();
    loop {
        match events.next()? {
            event::Key::Enter => {
                print!("Its Enter");
            }
            event::Key::Tab => {
                break;
            }
            _ => print!("Something else"),
        }
    }

    Ok(())
}
