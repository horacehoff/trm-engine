use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};

// WIP


pub enum KeyboardEvent {
    Char(char),
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    None,
}

pub fn read_char() -> KeyCode {
    if let Event::Key(KeyEvent {
                          code: c,
                          ..
                      }) = event::read().unwrap()
    {
        return c;
    } else {
        return KeyCode::Char('∅');
    }
}