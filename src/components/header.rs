use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{kunai::Kunai, ui::CurrentScreen};

pub fn render_header(frame: &mut Frame, header_rect: Rect, kunai: &mut Kunai) {
    match kunai.current_screen {
        CurrentScreen::TaskSelectionScreen => task_screen_header(frame, header_rect, kunai),
        CurrentScreen::MemoryEditingScreen => memedit_screen_header(frame, header_rect, kunai),
    }
}

fn task_screen_header(frame: &mut Frame, header_rect: Rect, kunai: &mut Kunai) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "KUNAI (A process memory editor)",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    let header_chunk;

    if kunai.tasks.pid_search || kunai.tasks.name_search {
        let search_title = if kunai.tasks.pid_search {
            "Search PID"
        } else {
            "Search Name"
        };

        let search_block = Block::new().borders(Borders::ALL).title(search_title);
        let search_input =
            Paragraph::new(Text::raw(&kunai.tasks.search_string)).block(search_block);

        header_chunk = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .split(header_rect);
        frame.render_widget(title, header_chunk[0]);
        frame.render_widget(search_input, header_chunk[1]);
    } else {
        header_chunk =
            Layout::new(Direction::Horizontal, [Constraint::Percentage(100)]).split(header_rect);

        frame.render_widget(title, header_chunk[0]);
    }
}

fn memedit_screen_header(frame: &mut Frame, header_rect: Rect, kunai: &mut Kunai) {
    let proc_info = Paragraph::new(Text::styled(
        format!("{} ({})", kunai.memedit.task.name, kunai.memedit.task.pid),
        Style::default().fg(Color::Green),
    ))
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(proc_info, header_rect);
}
