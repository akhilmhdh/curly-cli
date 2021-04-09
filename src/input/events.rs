use crate::input::Key;

use crossterm::event as crossterm_events;
use std::{sync::mpsc, thread, time::Duration};

#[derive(Debug)]
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
