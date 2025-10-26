use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

mod app;
mod ui;
pub mod core;

use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let app_result = run_app(&mut terminal).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Handle result
    match app_result {
        Ok(_) => {
            println!("✅ OWLSOL exited successfully");
            Ok(())
        }
        Err(err) => {
            eprintln!("❌ Error: {:?}", err);
            Err(err)
        }
    }
}

async fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    // Initialize app
    let mut app = match App::new().await {
        Ok(app) => app,
        Err(e) => {
            // If initialization fails, show error and exit
            eprintln!("Failed to initialize app: {}", e);
            return Err(e);
        }
    };

    // Main loop
    loop {
        // Draw UI
        terminal.draw(|f| ui::draw(f, &app))?;

        // Handle input with timeout
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        // Quit
                        return Ok(());
                    }
                    KeyCode::Esc => {
                        // Also quit on Escape
                        return Ok(());
                    }
                    KeyCode::Enter => {
                        // Execute swap if ready
                        if app.can_execute() {
                            if let Err(e) = app.execute_swap().await {
                                app.error = Some(format!("Swap failed: {}", e));
                                app.is_loading = false;
                            }
                        }
                    }
                    KeyCode::Tab => {
                        // Navigate to next field
                        app.next_field();
                    }
                    KeyCode::BackTab => {
                        // Navigate to previous field
                        app.previous_field();
                    }
                    KeyCode::Up => {
                        // Previous option
                        app.previous_option();
                    }
                    KeyCode::Down => {
                        // Next option
                        app.next_option();
                    }
                    KeyCode::Char(c) => {
                        // Handle character input
                        app.handle_input(c);
                    }
                    KeyCode::Backspace => {
                        // Handle backspace
                        app.handle_backspace();
                    }
                    _ => {}
                }
            }
        }

        // Auto-refresh data every 10 seconds
        if app.should_refresh() {
            if let Err(e) = app.refresh_data().await {
                app.error = Some(format!("Refresh failed: {}", e));
                app.is_loading = false;
                app.is_ready = false;
            }
        }
    }
}
