use anyhow::{Context, Result as AnyhowResult};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::{io::Result, time::Duration};

pub struct EventHandler {
    poll_timeout: Duration,
}

impl Default for EventHandler {
    fn default() -> Self {
        Self {
            poll_timeout: Duration::from_millis(500),
        }
    }
}

impl EventHandler {
    pub fn new(poll_timeout: Duration) -> Self {
        Self { poll_timeout }
    }

    pub fn poll_event(&self) -> Result<Option<Event>> {
        if event::poll(self.poll_timeout)? {
            event::read().map(Some)
        } else {
            Ok(None)
        }
    }

    pub fn handle_input(&self) -> AnyhowResult<EditorAction> {
        match self.poll_event().context("Failed to poll terminal event")? {
            Some(Event::Key(key)) => self.handle_key_event(key),
            Some(_) => Ok(EditorAction::Continue),
            None => Ok(EditorAction::Continue),
        }
    }

    fn handle_key_event(&self, key: KeyEvent) -> AnyhowResult<EditorAction> {
        match (key.code, key.modifiers) {
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(EditorAction::Quit),
            (KeyCode::Char('j'), _) => Ok(EditorAction::MoveCursor(Direction::Down)),
            (KeyCode::Char('k'), _) => Ok(EditorAction::MoveCursor(Direction::Up)),
            (KeyCode::Char('h'), _) => Ok(EditorAction::MoveCursor(Direction::Left)),
            (KeyCode::Char('l'), _) => Ok(EditorAction::MoveCursor(Direction::Right)),
            _ => Ok(EditorAction::Continue),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorAction {
    Quit,
    Continue,
    MoveCursor(Direction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_handler_default() {
        let handler = EventHandler::default();
        assert_eq!(handler.poll_timeout, Duration::from_millis(500));
    }

    #[test]
    fn test_event_handler_custom_timeout() {
        let timeout = Duration::from_secs(1);
        let handler = EventHandler::new(timeout);
        assert_eq!(handler.poll_timeout, timeout);
    }
}
