mod input;

use crate::input::{Events, Key};
use crossterm::terminal::enable_raw_mode;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // raw mode: https://docs.rs/crossterm/0.3.0/crossterm/raw/index.html
    enable_raw_mode()?;
    let events = Events::new();
    loop {
        match events.next()? {
            Key::Enter => {
                println!("Its Enter");
            }
            Key::Tab => {
                break;
            }
            _ => println!("Something else"),
        }
    }

    Ok(())
}
