use crossterm::event::{KeyEvent, KeyCode};

use crate::app::event::{Event, Key};

impl From<KeyEvent> for Event {
    fn from(value: KeyEvent) -> Self {
        let key = match value.code {
            KeyCode::Backspace => Key::ENTER,
            KeyCode::Enter => Key::ENTER,
            KeyCode::Left => Key::ENTER,
            KeyCode::Right => Key::ENTER,
            KeyCode::Up => Key::ENTER,
            KeyCode::Down => Key::ENTER,
            KeyCode::Home => Key::ENTER,
            KeyCode::End => Key::ENTER,
            KeyCode::PageUp => Key::ENTER,
            KeyCode::PageDown => Key::ENTER,
            KeyCode::Tab => Key::ENTER,
            KeyCode::BackTab => Key::ENTER,
            KeyCode::Delete => Key::ENTER,
            KeyCode::Insert => Key::ENTER,
            KeyCode::F(_) => Key::ENTER,
            KeyCode::Char(_) => Key::ENTER,
            KeyCode::Null => Key::ENTER,
            KeyCode::Esc => Key::ESC,
            KeyCode::CapsLock => Key::ENTER,
            KeyCode::ScrollLock => Key::ENTER,
            KeyCode::NumLock => Key::ENTER,
            KeyCode::PrintScreen => Key::ENTER,
            KeyCode::Pause => Key::ENTER,
            KeyCode::Menu => Key::ENTER,
            KeyCode::KeypadBegin => Key::ENTER,
            KeyCode::Media(_) => Key::ENTER,
            KeyCode::Modifier(_) => Key::ENTER,
        };

        Self::from(key)
    }
}
