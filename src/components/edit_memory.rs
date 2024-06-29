use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};

use crate::{
    kunai::Kunai,
    ui::SubScreen,
    utils::{centered_rect, num_to_hex},
};

pub fn render_memory_editor(frame: &mut Frame, body_rect: Rect, kunai: &mut Kunai) {
    if kunai.memedit.sub_screen == SubScreen::MemoryMaps {
        render_maps_table(frame, body_rect, kunai);
        return;
    }

    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Percentage(100)])
        .split(body_rect);

    let search_string_block = Paragraph::new(kunai.memedit.search_string.to_string())
        .block(
            Block::new()
                .borders(Borders::ALL)
                .title("Enter Search String"),
        )
        .style(Style::default());

    frame.render_widget(search_string_block, body_chunks[0]);

    render_search_table(frame, body_chunks[1], kunai);
}

fn render_search_table(frame: &mut Frame, table_rect: Rect, kunai: &mut Kunai) {
    let table_header = Row::new(vec!["START", "END", "VALUE", "MEM NAME"])
        .style(Style::new().bold())
        .bottom_margin(1);
    let column_widhts = [
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(30),
        Constraint::Percentage(30),
    ];

    let mut search_locations = Vec::new();

    for loc in &kunai.memedit.search_list {
        let row = Row::new(vec![
            num_to_hex(loc.start),
            num_to_hex(loc.end),
            loc.value.to_string(),
            loc.mem_info.name.to_string(),
        ]);

        search_locations.push(row);
    }

    let search_block = Block::default().style(Style::new()).borders(Borders::ALL);

    let table = Table::new(search_locations, column_widhts)
        .column_spacing(1)
        .style(Style::default())
        .header(table_header)
        .block(search_block)
        .highlight_style(
            Style::new()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Black)
                .bg(Color::Gray),
        );

    frame.render_stateful_widget(table, table_rect, &mut kunai.memedit.search_table_state);
}

fn render_maps_table(frame: &mut Frame, body_rect: Rect, kunai: &mut Kunai) {
    let table_header = Row::new(vec!["SELECTED", "START", "END", "PERMS", "NAME"])
        .style(Style::new().bold())
        .bottom_margin(1);

    let mut rows = Vec::new();

    for m in &kunai.memedit.task_mem.maps {
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
    frame.render_stateful_widget(table, area, &mut kunai.memedit.map_table_state);
}
