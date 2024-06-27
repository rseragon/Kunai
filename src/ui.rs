use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::{
    components::{
        edit_memory::render_memory_editor, footer::render_footer, header::render_header,
        task_selector::render_task_list,
    },
    kunai::Kunai,
};

#[derive(Debug)]
pub enum CurrentScreen {
    TaskSelectionScreen,
    MemoryEditingScreen,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SubScreen {
    MemoryMaps,
    MemorySearch,
    ValueEditing,
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
    render_header(frame, layout[0], kunai);

    // Render body
    match kunai.current_screen {
        CurrentScreen::TaskSelectionScreen => render_task_list(frame, layout[1], kunai),
        CurrentScreen::MemoryEditingScreen => render_memory_editor(frame, layout[1], kunai),
    };

    // Render footer
    render_footer(frame, layout[2], kunai);
}
