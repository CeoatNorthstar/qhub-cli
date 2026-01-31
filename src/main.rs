mod cli;
mod tui;
mod config;
mod api;
mod quantum;
mod auth;

use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io;
use std::time::Duration;

use cli::Args;
use config::Config;
use tui::{app::App, input, ui};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Ensure config directories exist
    Config::ensure_dirs()?;

    match args.command {
        Some(cli::Command::Run { file }) => {
            cli::commands::execute_run(&file).await?;
        }
        None => {
            run_tui().await?;
        }
    }

    Ok(())
}

async fn run_tui() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Main loop
    let tick_rate = Duration::from_millis(50);
    loop {
        // Check for AI responses
        app.check_ai_response();
        
        // Draw UI
        terminal.draw(|f| ui::render(f, &mut app))?;

        // Handle goodbye screen
        if app.show_exit_animation {
            app.exit_animation_frame += 1;
            // Show for ~1.5 seconds (30 frames at 50ms)
            if app.exit_animation_frame > 30 {
                break;
            }
            std::thread::sleep(tick_rate);
            continue;
        }

        if input::handle_events(&mut app, tick_rate)? {
            break;
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal - order matters!
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        DisableMouseCapture,
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}
