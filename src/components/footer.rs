use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{kunai::Kunai, ui::CurrentScreen};

pub fn render_footer(frame: &mut Frame, footer_rect: Rect, kunai: &Kunai) {
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(footer_rect);

    let help_message;

    // Rendering Error message
    if let Some(msg) = &kunai.ui_msg {
        help_message = "[Kunai] ".to_string() + msg;
    } else if let Some(msg) = &kunai.memedit.ui_msg {
        help_message = "[MemoryEditingScreen] ".to_string() + msg;
    } else if let Some(msg) = &kunai.tasks.ui_msg {
        help_message = "[TaskSelectionScreen] ".to_string() + msg;
    } else {
        help_message = match kunai.current_screen {
            CurrentScreen::TaskSelectionScreen => "Select a task!".to_string(),
            CurrentScreen::MemoryEditingScreen => "Memory Editor!".to_string(),
        };
    }

    let current_screen = Paragraph::new(Span::styled(
        help_message,
        Style::default().fg(Color::Yellow),
    ))
    .block(Block::default().borders(Borders::ALL));

    let key_hint = Paragraph::new(Span::styled(
        "? - to open hints",
        Style::default().fg(Color::Green),
    ))
    .block(Block::default().borders(Borders::ALL))
    .right_aligned();

    frame.render_widget(current_screen, footer_chunks[0]);
    frame.render_widget(key_hint, footer_chunks[1]);
}
