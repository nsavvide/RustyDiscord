use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
};

use std::io;

mod tui;
use crate::tui::{app::App, ui::render, input::handle_input};

mod config;
mod discord;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Initialize the TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    // Clear the terminal screen
    execute!(stdout, Clear(ClearType::All))?;

    // Enter alternate screen mode
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize the app asynchronously
    let mut app = App::new().await;

    // Main loop
    loop {
        terminal.draw(|f| render(f, &app))?;

        if handle_input(&mut app) {
            break;
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
