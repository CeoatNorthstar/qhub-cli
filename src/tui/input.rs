use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind};
use std::time::Duration;

use super::app::{App, InputMode};

pub fn handle_events(app: &mut App, timeout: Duration) -> Result<bool> {
    if event::poll(timeout)? {
        match event::read()? {
            Event::Key(key) => {
                // Only handle key press events, ignore release/repeat to prevent double input
                if key.kind != KeyEventKind::Press {
                    return Ok(false);
                }
                
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(true);
                        }
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(true);
                        }
                        KeyCode::Enter => {
                            app.submit_input();
                        }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Up => {
                            app.scroll_up();
                        }
                        KeyCode::Down => {
                            app.scroll_down();
                        }
                        KeyCode::PageUp => {
                            for _ in 0..10 {
                                app.scroll_up();
                            }
                        }
                        KeyCode::PageDown => {
                            for _ in 0..10 {
                                app.scroll_down();
                            }
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::Enter if key.modifiers.contains(KeyModifiers::SHIFT) => {
                            app.input.push('\n');
                        }
                        KeyCode::Enter => {
                            app.submit_input();
                        }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        _ => {}
                    },
                }
            }
            Event::Mouse(mouse) => {
                match mouse.kind {
                    MouseEventKind::ScrollUp => {
                        for _ in 0..3 {
                            app.scroll_up();
                        }
                    }
                    MouseEventKind::ScrollDown => {
                        for _ in 0..3 {
                            app.scroll_down();
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    Ok(false)
}
