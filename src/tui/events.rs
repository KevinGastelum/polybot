//! TUI event handling.

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;

/// Poll for keyboard events with timeout.
pub fn poll_event(timeout: Duration) -> Option<Event> {
    if event::poll(timeout).ok()? {
        event::read().ok()
    } else {
        None
    }
}

/// Handle a keyboard event.
pub fn handle_key_event(app: &mut super::app::App, event: Event) {
    if let Event::Key(key) = event {
        // Only handle key press events (not release)
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char(c) => app.on_key(c),
            code => app.on_special_key(code),
        }
    }
}
