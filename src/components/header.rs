use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_header(frame: &mut Frame, header_rect: Rect) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "KUNAI (A process memory editor)",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, header_rect);
}
