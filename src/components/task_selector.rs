use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{block::Position, Block, Borders, Row, Table},
    Frame,
};

use crate::kunai::Kunai;

pub fn render_task_list(frame: &mut Frame, body_rect: Rect, kunai: &mut Kunai) {
    let table_header = Row::new(vec!["PID", "NAME", "STATE", "CMDLINE"])
        .style(Style::new().bold())
        .bottom_margin(1);

    let mut rows = Vec::new();

    // Searching
    if kunai.tasks.name_search {
        for t in &kunai.tasks.task_list {
            if t.name.contains(&kunai.tasks.search_string) {
                rows.push(Row::new(vec![
                    t.pid.clone(),
                    t.name.clone(),
                    t.state.clone(),
                    t.cmdline.clone(),
                ]))
            }
        }
    } else if kunai.tasks.pid_search {
        for t in &kunai.tasks.task_list {
            if t.pid.contains(&kunai.tasks.search_string) {
                rows.push(Row::new(vec![
                    t.pid.clone(),
                    t.name.clone(),
                    t.state.clone(),
                    t.cmdline.clone(),
                ]))
            }
        }
    } else {
        // Show everything!
        for t in &kunai.tasks.task_list {
            // Why Clone?
            // Cuz when I referesh the task list, the references would be gone
            rows.push(Row::new(vec![
                t.pid.clone(),
                t.name.clone(),
                t.state.clone(),
                t.cmdline.clone(),
            ]))
        }
    }

    let column_widhts = [
        Constraint::Length(10),
        Constraint::Percentage(35),
        Constraint::Length(15),
        Constraint::Min(15),
    ];

    let table = Table::new(rows, column_widhts)
        .column_spacing(1)
        .style(Style::default())
        .header(table_header)
        .block(
            Block::new()
                .title("Tasks")
                .borders(Borders::ALL)
                .title_position(Position::Top),
        )
        .highlight_style(
            Style::new()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Black)
                .bg(Color::Gray),
        );

    frame.render_stateful_widget(table, body_rect, &mut kunai.tasks.table_state);
}
