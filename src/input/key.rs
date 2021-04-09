use crossterm::event as crossterm_events;
use std::fmt;

// copied from crossterm keycodes
#[derive(Debug)]
pub enum Key {
    /// Backspace key.
    Backspace,
    /// Enter key.
    Enter,
    /// Left arrow key.
    Left,
    /// Right arrow key.
    Right,
    /// Up arrow key.
    Up,
    /// Down arrow key.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page up key.
    PageUp,
    /// Page dow key.
    PageDown,
    /// Tab key.
    Tab,
    /// Shift + Tab key.
    BackTab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// F key.
    ///
    /// `KeyEvent::F(1)` represents F1 key, etc.
    F(u8),
    /// A character.
    ///
    /// `KeyEvent::Char('c')` represents `c` character, etc.
    Char(char),
    /// Null.
    Null,
    /// Escape key.
    Esc,
    // Unknown,
}

impl fmt::Display for Key {
    /**
     * Trait to print
     */
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Key::Tab | Key::Enter => write!(f, "Tab | Enter <{:?}>", self),
            Key::Char(c) => write!(f, "Char {}", c),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Key {
    /**
     * why: crossterm event returns struct too tough to compare
     * to solve this convert keystroke struct to enum
     * much simpler and easier to work with
     */
    pub fn from(key_event: crossterm_events::KeyEvent) -> Self {
        match key_event {
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::Backspace,
                ..
            } => Key::Backspace,
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::Enter,
                ..
            } => Key::Enter,
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::Left,
                ..
            } => Key::Left,
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::Right,
                ..
            } => Key::Right,
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::Up,
                ..
            } => Key::Up,
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::Down,
                ..
            } => Key::Down,
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::Home,
                ..
            } => Key::Home,
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::End,
                ..
            } => Key::End,
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::PageUp,
                ..
            } => Key::PageUp,
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::PageDown,
                ..
            } => Key::PageDown,
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::Tab,
                ..
            } => Key::Tab,
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::BackTab,
                ..
            } => Key::BackTab,
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::Delete,
                ..
            } => Key::Delete,
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::Insert,
                ..
            } => Key::Insert,
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::F(u8),
                ..
            } => Key::F(u8),
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::Null,
                ..
            } => Key::Null,
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::Esc,
                ..
            } => Key::Esc,
            crossterm_events::KeyEvent {
                code: crossterm_events::KeyCode::Char(c),
                ..
            } => Key::Char(c),
        }
    }
}
