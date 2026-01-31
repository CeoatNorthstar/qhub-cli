use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use super::app::{App, MessageRole};

// Minimal color palette - muted and clean
const MUTED_WHITE: Color = Color::Rgb(200, 200, 200);
const DIM_GRAY: Color = Color::Rgb(100, 100, 100);
const SOFT_BLUE: Color = Color::Rgb(130, 160, 200);
const SOFT_GREEN: Color = Color::Rgb(120, 180, 120);
const SOFT_RED: Color = Color::Rgb(200, 100, 100);
const CYAN: Color = Color::Rgb(0, 205, 205);  // Smooth cyan

pub fn render(frame: &mut Frame, app: &mut App) {
    // Show goodbye screen
    if app.show_exit_animation {
        render_goodbye(frame);
        return;
    }
    
    // Calculate suggestion height dynamically
    let suggestion_height = if app.show_suggestions {
        (app.suggestions.len().min(5) + 2) as u16  // Max 5 suggestions + border
    } else {
        0
    };
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),              // Header - minimal
            Constraint::Min(10),                // Messages
            Constraint::Length(3),              // Input
            Constraint::Length(suggestion_height), // Suggestions (dynamic)
            Constraint::Length(1),              // Status bar
        ])
        .split(frame.area());

    render_header(frame, chunks[0]);
    render_messages(frame, app, chunks[1]);
    render_input(frame, app, chunks[2]);
    
    // Render suggestions if showing
    if app.show_suggestions {
        render_suggestions(frame, app, chunks[3]);
    }
    
    render_status_bar(frame, app, chunks[4]);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let header = Paragraph::new(Line::from(vec![
        Span::styled("qhub", Style::default().fg(CYAN).add_modifier(Modifier::BOLD)),
    ]));
    
    frame.render_widget(header, area);
}

fn render_goodbye(frame: &mut Frame) {
    let area = frame.area();
    let text = vec![
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled("goodbye", Style::default().fg(CYAN))),
        Line::from(""),
    ];
    
    let paragraph = Paragraph::new(text)
        .alignment(ratatui::layout::Alignment::Center);
    
    frame.render_widget(paragraph, area);
}

fn render_messages(frame: &mut Frame, app: &mut App, area: Rect) {
    let inner_height = area.height.saturating_sub(2) as usize;
    
    let mut all_lines: Vec<Line> = Vec::new();
    
    for message in &app.messages {
        let (prefix, prefix_style) = match message.role {
            MessageRole::User => ("> ", Style::default().fg(SOFT_GREEN)),
            MessageRole::Assistant => ("  ", Style::default().fg(SOFT_BLUE)),
            MessageRole::System => ("  ", Style::default().fg(DIM_GRAY)),
            MessageRole::Error => ("! ", Style::default().fg(SOFT_RED)),
        };

        let content_style = match message.role {
            MessageRole::User => Style::default().fg(MUTED_WHITE),
            MessageRole::Assistant => Style::default().fg(MUTED_WHITE),
            MessageRole::System => Style::default().fg(DIM_GRAY),
            MessageRole::Error => Style::default().fg(SOFT_RED),
        };

        let mut in_code_block = false;
        
        for (i, line) in message.content.lines().enumerate() {
            if line.starts_with("```") {
                in_code_block = !in_code_block;
                if in_code_block {
                    all_lines.push(Line::from(Span::styled("", Style::default())));
                }
                continue;
            }
            
            if in_code_block {
                all_lines.push(Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(line.to_string(), Style::default().fg(SOFT_BLUE)),
                ]));
            } else {
                let line_prefix = if i == 0 { prefix } else { "  " };
                all_lines.push(Line::from(vec![
                    Span::styled(line_prefix, prefix_style),
                    Span::styled(line.to_string(), content_style),
                ]));
            }
        }
        
        all_lines.push(Line::from(""));
    }
    
    // Show loading indicator
    if app.is_loading {
        all_lines.push(Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled("● ", Style::default().fg(CYAN).add_modifier(Modifier::SLOW_BLINK)),
            Span::styled("thinking...", Style::default().fg(DIM_GRAY)),
        ]));
    }

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
                .borders(Borders::TOP)
                .border_style(Style::default().fg(DIM_GRAY))
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(messages_widget, area);

    if total_lines > inner_height {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None);
        
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
        Span::styled("...", Style::default().fg(DIM_GRAY))
    } else if app.input.is_empty() {
        Span::styled("Type a message...", Style::default().fg(DIM_GRAY))
    } else {
        Span::styled(&app.input, Style::default().fg(MUTED_WHITE))
    };

    let input_widget = Paragraph::new(Line::from(vec![
        Span::styled("> ", Style::default().fg(DIM_GRAY)),
        input_text,
    ]))
    .block(
        Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(DIM_GRAY))
    );

    frame.render_widget(input_widget, area);

    if !app.is_loading {
        let cursor_x = area.x + 2 + app.input.len() as u16;
        let cursor_y = area.y + 1;
        if cursor_x < area.x + area.width - 1 {
            frame.set_cursor_position((cursor_x, cursor_y));
        }
    }
}

fn render_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let status_parts: Vec<Span> = vec![
        if let Some(email) = &app.user_email {
            Span::styled(email.as_str(), Style::default().fg(DIM_GRAY))
        } else {
            Span::styled("not logged in", Style::default().fg(DIM_GRAY))
        },
        Span::styled(" · ", Style::default().fg(DIM_GRAY)),
        Span::styled("esc to exit", Style::default().fg(DIM_GRAY)),
    ];

    let status_widget = Paragraph::new(Line::from(status_parts));
    frame.render_widget(status_widget, area);
}

fn render_suggestions(frame: &mut Frame, app: &App, area: Rect) {
    if area.height < 2 {
        return; // Not enough space
    }
    
    // Create suggestion lines with highlighting for selected item
    let suggestions: Vec<Line> = app.suggestions
        .iter()
        .enumerate()
        .take(5)  // Max 5 visible suggestions
        .map(|(i, suggestion)| {
            let is_selected = i == app.selected_suggestion;
            let style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(CYAN)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(MUTED_WHITE)
            };
            
            let prefix = if is_selected { " ▶ " } else { "   " };
            Line::from(vec![
                Span::raw(prefix),
                Span::styled(suggestion, style),
            ])
        })
        .collect();
    
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(CYAN))
        .title(Span::styled(
            " Suggestions (↑↓ to navigate, Tab to select) ",
            Style::default().fg(CYAN).add_modifier(Modifier::BOLD),
        ));
    
    let paragraph = Paragraph::new(suggestions)
        .block(block);
    
    frame.render_widget(paragraph, area);
}
