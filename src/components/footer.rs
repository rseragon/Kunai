use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_footer(frame: &mut Frame, footer_rect: Rect) {
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(footer_rect);

    let current_screen = Paragraph::new(Span::styled(
        "Main Screen!",
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
