use crossterm::event::{self, Event, KeyCode};
use crate::tui::app::App;

pub fn handle_input(app: &mut App) -> bool {
    if let Event::Key(key) = event::read().unwrap() {
        match key.code {
            KeyCode::Char('q') => return true, // Quit
            KeyCode::Tab => app.next_tab(),
            KeyCode::BackTab => app.previous_tab(),
            KeyCode::Down => app.next_channel(),
            KeyCode::Up => app.previous_channel(),
            _ => {}
        }
    }
    false
}
