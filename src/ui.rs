use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::{
    components::{footer::render_footer, header::render_header, task_selector::render_task_list},
    kunai::Kunai,
};

#[derive(Debug)]
pub enum CurrentScreen {
    TaskSelection,
}

pub fn render_ui(frame: &mut Frame, kunai: &mut Kunai) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.size());

    // Render title
    render_header(frame, layout[0]);

    // Render body
    match kunai.current_screen {
        CurrentScreen::TaskSelection => render_task_list(frame, layout[1], kunai),
    };

    // Render footer
    render_footer(frame, layout[2]);
}
