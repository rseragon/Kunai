use core::panic;
use std::{borrow::Cow, usize};

use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn is_numeric(n: &str) -> bool {
    n.parse::<f64>().is_ok()
}

// TODO: Should this be i64?
pub fn num_to_hex(n: i64) -> String {
    format!("{:x}", n)
}

pub fn bytes_to_string(value: Vec<u8>) -> String {
    match String::from_utf8(value.clone()) {
        Ok(s) => s,
        Err(_) => {
            let mut s = String::new();
            for v in value {
                s = format!("{}{}", s, to_ascii(v)); // Fuck Strings
            }
            s
        }
    }
}

fn to_ascii(i: u8) -> String {
    // Taken from https://www.cs.cmu.edu/~pattis/15-1XX/common/handouts/ascii.html
    match i {
        x @ 0..=32 => [
            "NUL", "SOH", "STX", "ETX", "EOT", "ENQ", "ACK", "BEL", "BS", "TAB", "LF", "VT", "FF",
            "CR", "SO", "SI", "DLE", "DC1", "DC2", "DC3", "DC4", "NAK", "SYN", "ETB", "CAN", "EM",
            "SUB", "ESC", "FS", "GS", "RS", "US", "SPACE",
        ][x as usize]
            .into(),
        x @ 33..=126 => format!("{}", x as u8 as char).into(),
        127 => "DEL".into(),
        _ => "\0".into(),
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
