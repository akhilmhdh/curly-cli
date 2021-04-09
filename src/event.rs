use crossterm::event::{poll, read};
use std;
use std::sync::mpsc;

pub enum Key {
    Enter,
    Tab,
    Char(char),
}

pub enum Event<I> {
    Input(I),
}

pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    tx: mpsc::Sender<Event<Key>>,
}

impl Events {
    pub fn new() -> Events {
        let (tx, rx) = mpsc::channel();
        Events { rx, tx }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}
