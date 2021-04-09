use crossterm::event as crossterm_events;
use std::fmt;

#[derive(Debug)]
pub enum Key {
    Enter,
    Tab,
    Char(char),
    Unknown,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Key::Tab | Key::Enter => write!(f, "Tab | Enter <{:?}>", self),
            Key::Char(c) => write!(f, "Char {}", c),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Key {
    pub fn from(key_event: crossterm_events::KeyEvent) -> Self {
        match key_event {
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::Tab,
                ..
            } => Key::Tab,
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::Enter,
                ..
            } => Key::Tab,
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::Char(c),
                ..
            } => Key::Char(c),
            _ => Key::Unknown,
        }
    }
}
