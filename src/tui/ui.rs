use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use super::app::{App, Message, MessageRole};

const QHUB_PURPLE: Color = Color::Rgb(138, 43, 226);
const QHUB_CYAN: Color = Color::Rgb(0, 255, 255);
const QHUB_GREEN: Color = Color::Rgb(0, 255, 127);
const QHUB_YELLOW: Color = Color::Rgb(255, 215, 0);
const QHUB_RED: Color = Color::Rgb(255, 99, 71);
const QHUB_GRAY: Color = Color::Rgb(128, 128, 128);

pub fn render(frame: &mut Frame, app: &mut App) {
    // If exit animation is playing, render that instead
    if app.show_exit_animation {
        render_exit_animation(frame, app);
        return;
    }
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Messages
            Constraint::Length(5),  // Input
            Constraint::Length(1),  // Status bar
        ])
        .split(frame.area());

    render_header(frame, chunks[0]);
    render_messages(frame, app, chunks[1]);
    render_input(frame, app, chunks[2]);
    render_status_bar(frame, app, chunks[3]);
}

fn render_exit_animation(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    
    let exit_art = r#"
    ╔═══════════════════════════════════════════════════════════════════╗
    ║                                                                   ║
    ║      ████████╗██╗  ██╗ ██████╗ ██╗   ██╗                         ║
    ║      ╚══██╔══╝██║  ██║██╔═══██╗██║   ██║                         ║
    ║         ██║   ███████║██║   ██║██║   ██║                         ║
    ║         ██║   ██╔══██║██║   ██║██║   ██║                         ║
    ║         ██║   ██║  ██║╚██████╔╝╚██████╔╝                         ║
    ║         ╚═╝   ╚═╝  ╚═╝ ╚═════╝  ╚═════╝                          ║
    ║                                                                   ║
    ║       ██████╗ ██████╗ ██████╗ ███████╗██████╗ ███████╗██████╗    ║
    ║      ██╔═══██╗██╔══██╗██╔══██╗██╔════╝██╔══██╗██╔════╝██╔══██╗   ║
    ║      ██║   ██║██████╔╝██║  ██║█████╗  ██████╔╝█████╗  ██║  ██║   ║
    ║      ██║   ██║██╔══██╗██║  ██║██╔══╝  ██╔══██╗██╔══╝  ██║  ██║   ║
    ║      ╚██████╔╝██║  ██║██████╔╝███████╗██║  ██║███████╗██████╔╝   ║
    ║       ╚═════╝ ╚═╝  ╚═╝╚═════╝ ╚══════╝╚═╝  ╚═╝╚══════╝╚═════╝    ║
    ║                                                                   ║
    ║          ⚛  Your quantum journey awaits another day  ⚛           ║
    ║                                                                   ║
    ║                    We shall close as commanded.                   ║
    ║                                                                   ║
    ╚═══════════════════════════════════════════════════════════════════╝
    "#;
    
    let lines: Vec<Line> = exit_art
        .lines()
        .map(|line| {
            Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(QHUB_PURPLE)
            ))
        })
        .collect();
    
    let paragraph = Paragraph::new(lines)
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default());
    
    frame.render_widget(paragraph, area);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let header = Paragraph::new(Line::from(vec![
        Span::styled("  ⚛ ", Style::default().fg(QHUB_CYAN)),
        Span::styled("QHub", Style::default().fg(QHUB_PURPLE).add_modifier(Modifier::BOLD)),
        Span::styled(" │ ", Style::default().fg(QHUB_GRAY)),
        Span::styled("Quantum Computing + AI", Style::default().fg(QHUB_GRAY)),
    ]))
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(QHUB_GRAY))
    );
    
    frame.render_widget(header, area);
}

fn render_messages(frame: &mut Frame, app: &mut App, area: Rect) {
    let inner_height = area.height.saturating_sub(2) as usize;
    
    // Build all lines from messages
    let mut all_lines: Vec<Line> = Vec::new();
    
    for message in &app.messages {
        let (prefix, style) = match message.role {
            MessageRole::User => ("You", Style::default().fg(QHUB_GREEN).add_modifier(Modifier::BOLD)),
            MessageRole::Assistant => ("QHub", Style::default().fg(QHUB_PURPLE).add_modifier(Modifier::BOLD)),
            MessageRole::System => ("", Style::default().fg(QHUB_CYAN)),
            MessageRole::Error => ("Error", Style::default().fg(QHUB_RED).add_modifier(Modifier::BOLD)),
        };

        if !prefix.is_empty() {
            all_lines.push(Line::from(vec![
                Span::styled(format!("{}: ", prefix), style),
            ]));
        }

        // Parse content for code blocks
        let mut in_code_block = false;
        let mut code_lang = String::new();
        
        for line in message.content.lines() {
            if line.starts_with("```") {
                if !in_code_block {
                    // Starting code block
                    in_code_block = true;
                    code_lang = line.trim_start_matches('`').to_string();
                    let lang_display = if code_lang.is_empty() { "code" } else { &code_lang };
                    all_lines.push(Line::from(vec![
                        Span::styled("┌─", Style::default().fg(QHUB_CYAN)),
                        Span::styled(format!(" {} ", lang_display), Style::default().fg(QHUB_YELLOW).add_modifier(Modifier::BOLD)),
                        Span::styled("─".repeat(50), Style::default().fg(QHUB_CYAN)),
                    ]));
                } else {
                    // Ending code block
                    in_code_block = false;
                    code_lang.clear();
                    all_lines.push(Line::from(Span::styled(
                        "└".to_string() + &"─".repeat(60),
                        Style::default().fg(QHUB_CYAN)
                    )));
                }
            } else if in_code_block {
                // Code content - special styling
                all_lines.push(Line::from(vec![
                    Span::styled("│ ", Style::default().fg(QHUB_CYAN)),
                    Span::styled(line.to_string(), Style::default().fg(Color::Rgb(180, 220, 255)).add_modifier(Modifier::ITALIC)),
                ]));
            } else {
                // Parse markdown in regular content
                let base_style = match message.role {
                    MessageRole::User => Style::default().fg(Color::White),
                    MessageRole::Assistant => Style::default().fg(Color::White),
                    MessageRole::System => Style::default().fg(QHUB_CYAN),
                    MessageRole::Error => Style::default().fg(QHUB_RED),
                };
                all_lines.push(parse_markdown_line(line, base_style));
            }
        }
        
        all_lines.push(Line::from("")); // Empty line between messages
    }

    // Calculate scroll
    let total_lines = all_lines.len();
    let max_scroll = total_lines.saturating_sub(inner_height);
    
    if app.scroll_offset > max_scroll {
        app.scroll_offset = max_scroll;
    }

    let visible_lines: Vec<Line> = all_lines
        .into_iter()
        .skip(app.scroll_offset)
        .take(inner_height)
        .collect();

    let messages_widget = Paragraph::new(visible_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(QHUB_GRAY))
                .title(Span::styled(" Chat ", Style::default().fg(QHUB_PURPLE)))
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(messages_widget, area);

    // Render scrollbar if needed
    if total_lines > inner_height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"));
        
        let mut scrollbar_state = ScrollbarState::new(max_scroll)
            .position(app.scroll_offset);
        
        frame.render_stateful_widget(
            scrollbar,
            area.inner(ratatui::layout::Margin { vertical: 1, horizontal: 0 }),
            &mut scrollbar_state,
        );
    }
}

fn render_input(frame: &mut Frame, app: &App, area: Rect) {
    let input_text = if app.is_loading {
        Span::styled(
            "⏳ Thinking...",
            Style::default().fg(QHUB_YELLOW)
        )
    } else if app.input.is_empty() {
        Span::styled(
            "Type a message or /help for commands...",
            Style::default().fg(QHUB_GRAY)
        )
    } else {
        Span::styled(&app.input, Style::default().fg(Color::White))
    };
    
    let border_color = if app.is_loading { QHUB_YELLOW } else { QHUB_PURPLE };

    let input_widget = Paragraph::new(Line::from(input_text))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .title(Span::styled(" > ", Style::default().fg(QHUB_GREEN).add_modifier(Modifier::BOLD)))
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(input_widget, area);

    // Show cursor
    let cursor_x = area.x + 1 + app.input.len() as u16;
    let cursor_y = area.y + 1;
    if cursor_x < area.x + area.width - 1 {
        frame.set_cursor_position((cursor_x, cursor_y));
    }
}

fn render_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let user_status = if let Some(email) = &app.user_email {
        Span::styled(format!(" {} ", email), Style::default().fg(QHUB_GREEN))
    } else {
        Span::styled(" Not logged in ", Style::default().fg(QHUB_YELLOW))
    };

    let tier_style = match app.user_tier.as_str() {
        "pro" => Style::default().fg(QHUB_PURPLE).add_modifier(Modifier::BOLD),
        _ => Style::default().fg(QHUB_GRAY),
    };
    let tier = Span::styled(format!(" {} ", app.user_tier.to_uppercase()), tier_style);

    let connection = if app.is_connected {
        Span::styled(" ● Connected ", Style::default().fg(QHUB_GREEN))
    } else {
        Span::styled(" ○ Offline ", Style::default().fg(QHUB_GRAY))
    };

    let status_line = Line::from(vec![
        Span::styled("│", Style::default().fg(QHUB_GRAY)),
        user_status,
        Span::styled("│", Style::default().fg(QHUB_GRAY)),
        tier,
        Span::styled("│", Style::default().fg(QHUB_GRAY)),
        connection,
        Span::styled("│", Style::default().fg(QHUB_GRAY)),
        Span::styled(" Ctrl+C to exit ", Style::default().fg(QHUB_GRAY)),
    ]);

    let status_widget = Paragraph::new(status_line);
    frame.render_widget(status_widget, area);
}

/// Parse a line of text for markdown formatting and return styled spans
fn parse_markdown_line<'a>(line: &'a str, base_style: Style) -> Line<'a> {
    let mut spans: Vec<Span> = Vec::new();
    let mut chars = line.chars().peekable();
    let mut current_text = String::new();
    
    while let Some(c) = chars.next() {
        match c {
            // Bold: **text** or __text__
            '*' | '_' if chars.peek() == Some(&c) => {
                // Push any accumulated text
                if !current_text.is_empty() {
                    spans.push(Span::styled(current_text.clone(), base_style));
                    current_text.clear();
                }
                
                chars.next(); // consume second * or _
                let delimiter = c;
                let mut bold_text = String::new();
                
                while let Some(bc) = chars.next() {
                    if bc == delimiter && chars.peek() == Some(&delimiter) {
                        chars.next(); // consume closing delimiter
                        break;
                    }
                    bold_text.push(bc);
                }
                
                if !bold_text.is_empty() {
                    spans.push(Span::styled(
                        bold_text,
                        base_style.add_modifier(Modifier::BOLD).fg(QHUB_YELLOW)
                    ));
                }
            }
            // Inline code: `code`
            '`' => {
                if !current_text.is_empty() {
                    spans.push(Span::styled(current_text.clone(), base_style));
                    current_text.clear();
                }
                
                let mut code_text = String::new();
                while let Some(cc) = chars.next() {
                    if cc == '`' {
                        break;
                    }
                    code_text.push(cc);
                }
                
                if !code_text.is_empty() {
                    spans.push(Span::styled(
                        format!(" {} ", code_text),
                        Style::default().fg(Color::Rgb(180, 220, 255)).bg(Color::Rgb(40, 40, 50))
                    ));
                }
            }
            // Headers: # Header
            '#' if current_text.is_empty() && spans.is_empty() => {
                let mut header_level = 1;
                while chars.peek() == Some(&'#') {
                    chars.next();
                    header_level += 1;
                }
                // Skip space after #
                if chars.peek() == Some(&' ') {
                    chars.next();
                }
                
                let rest: String = chars.collect();
                let header_style = match header_level {
                    1 => base_style.fg(QHUB_PURPLE).add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                    2 => base_style.fg(QHUB_CYAN).add_modifier(Modifier::BOLD),
                    _ => base_style.fg(QHUB_GREEN).add_modifier(Modifier::BOLD),
                };
                
                return Line::from(Span::styled(rest, header_style));
            }
            // Bullet points: - or *
            '-' | '*' if current_text.is_empty() && spans.is_empty() && chars.peek() == Some(&' ') => {
                chars.next(); // consume space
                spans.push(Span::styled("  • ", Style::default().fg(QHUB_CYAN)));
            }
            // Regular character
            _ => {
                current_text.push(c);
            }
        }
    }
    
    // Push any remaining text
    if !current_text.is_empty() {
        spans.push(Span::styled(current_text, base_style));
    }
    
    if spans.is_empty() {
        Line::from("")
    } else {
        Line::from(spans)
    }
}
