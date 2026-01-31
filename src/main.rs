mod cli;
mod tui;
mod config;
mod api;
mod quantum;

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
    // Load environment variables from .env file (development)
    // In production, env vars are set via deployment system
    dotenv::dotenv().ok();
    
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
    // Setup terminal with panic handler for proper cleanup
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // Restore terminal on panic
        let _ = disable_raw_mode();
        let _ = execute!(
            io::stdout(),
            DisableMouseCapture,
            LeaveAlternateScreen
        );
        original_hook(panic_info);
    }));
    
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
        
        // Check for auth responses
        app.check_auth_response();
        
        // Draw UI
        terminal.draw(|f| ui::render(f, &mut app))?;

        // Check for exit
        if app.should_quit || input::handle_events(&mut app, tick_rate)? {
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
    
    // Explicit ANSI reset to prevent escape code leakage
    // This ensures any residual formatting is cleared
    print!("\x1b[0m");
    std::io::Write::flush(&mut std::io::stdout())?;

    Ok(())
}
