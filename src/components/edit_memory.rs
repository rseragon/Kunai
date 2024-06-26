use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

use crate::{kunai::Kunai, utils::centered_rect};

pub fn render_memory_editor(frame: &mut Frame, body_rect: Rect, kunai: &mut Kunai) {
    if kunai.memedit.show_maps {
        render_maps_table(frame, body_rect, kunai);
    }
}

fn render_maps_table(frame: &mut Frame, body_rect: Rect, kunai: &mut Kunai) {
    let table_header = Row::new(vec!["SELECTED", "START", "END", "PERMS", "NAME"])
        .style(Style::new().bold())
        .bottom_margin(1);

    let mut rows = Vec::new();

    for m in &kunai.memedit.task_mem.map {
        rows.push(Row::new(vec![
            match m.should_search {
                true => "Y".to_string(),
                false => "N".to_string(),
            },
            m.start.to_string(),
            m.end.to_string(),
            m.perms.to_string(),
            m.name.to_string(),
        ]));
    }

    let column_widhts = [
        Constraint::Percentage(10),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(10),
        Constraint::Percentage(40),
    ];

    let maps_block = Block::default()
        .title("Maps")
        .borders(Borders::ALL)
        .style(Style::default());

    let table = Table::new(rows, column_widhts)
        .column_spacing(1)
        .style(Style::default())
        .header(table_header)
        .block(maps_block)
        .highlight_style(
            Style::new()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Black)
                .bg(Color::Gray),
        );

    let area = centered_rect(80, 90, body_rect);
    frame.render_widget(table, area);
}
