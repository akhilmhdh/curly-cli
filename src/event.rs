use crossterm::event as crossterm_events;
use std::fmt;
use std::{sync::mpsc, thread, time::Duration};

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

pub struct Events {
    rx: mpsc::Receiver<Key>,
    tx: mpsc::Sender<Key>,
}

impl Events {
    pub fn new() -> Events {
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();

        thread::spawn(move || loop {
            if crossterm_events::poll(Duration::from_millis(100)).unwrap() {
                if let crossterm_events::Event::Key(key) = crossterm_events::read().unwrap() {
                    let key = Key::from(key);
                    event_tx.send(key).unwrap();
                }
            }
        });

        Events { rx, tx }
    }

    pub fn next(&self) -> Result<Key, mpsc::RecvError> {
        self.rx.recv()
    }
}
