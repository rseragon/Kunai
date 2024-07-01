use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, Borders, Clear, Padding, Paragraph, Row, Table},
    Frame,
};

use crate::{
    kunai::Kunai,
    ui::SubScreen,
    utils::{centered_rect, num_to_hex},
};

pub fn render_memory_editor(frame: &mut Frame, body_rect: Rect, kunai: &mut Kunai) {
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

    if kunai.memedit.sub_screen == SubScreen::MemoryMaps {
        render_maps_table(frame, body_rect, kunai);
    } else if kunai.memedit.sub_screen == SubScreen::ValueEditing {
        render_value_editor(frame, body_rect, kunai);
    }
}

pub fn render_value_editor(frame: &mut Frame, popup_rect: Rect, kunai: &mut Kunai) {
    let area = centered_rect(40, 60, popup_rect);

    let editor_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(4), Constraint::Percentage(100)])
        .split(area);

    let selected_value = match kunai.memedit.search_table_state.selected() {
        Some(idx) => kunai.memedit.search_list[idx].value.to_string(),
        None => "None".to_string(),
    };

    let curr_value = Paragraph::new(Text::styled(selected_value, Style::default().dark_gray()))
        .style(Style::default())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Selected Value"),
        );

    let new_value = Paragraph::new(Text::styled(
        &kunai.memedit.new_value,
        Style::default().slow_blink(),
    ))
    .style(Style::default())
    .block(Block::default().borders(Borders::ALL).title("New Value"));

    frame.render_widget(Clear, area);

    frame.render_widget(curr_value, editor_chunks[0]);
    frame.render_widget(new_value, editor_chunks[1]);
}

fn render_search_table(frame: &mut Frame, table_rect: Rect, kunai: &mut Kunai) {
    let table_header = Row::new(vec!["START", "END", "VALUE", "MEM NAME"])
        .style(Style::new().bold())
        .bottom_margin(1);
    let column_widhts = [
        Constraint::Min(15),
        Constraint::Min(15),
        Constraint::Percentage(40),
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
            num_to_hex(m.start),
            num_to_hex(m.end),
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
        .border_style(Style::default().fg(Color::Yellow))
        .style(Style::default());

    let table = Table::new(rows, column_widhts)
        .column_spacing(1)
        .style(Style::default().bg(Color::Black))
        .header(table_header)
        .block(maps_block)
        .highlight_style(
            Style::new()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Black)
                .bg(Color::Gray),
        );

    let area = centered_rect(80, 90, body_rect);

    frame.render_widget(Clear, area); // Clear the screen where the pop is rendered

    frame.render_stateful_widget(table, area, &mut kunai.memedit.map_table_state);
}
