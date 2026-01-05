//! TUI binary entry point.

use std::io;
use std::time::Duration;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use polymarket_kalshi_arbitrage_bot::tui::{app::App, events, ui};

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Main loop
    loop {
        // Draw UI
        terminal.draw(|frame| ui::draw(frame, &app))?;

        // Handle events with 100ms timeout
        if let Some(event) = events::poll_event(Duration::from_millis(100)) {
            events::handle_key_event(&mut app, event);
        }

        // Check if we should quit
        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    println!("Thanks for using Polymarket-Kalshi Arbitrage Bot!");
    println!("Final balance: ${:.2}", app.engine.portfolio.total_value());

    Ok(())
}
