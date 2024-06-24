use std::{error::Error, io::Stdout};

use event_handler::handle_keypress;
use kunai::Kunai;
use ratatui::{backend::CrosstermBackend, Terminal};
use ui::render_ui;

mod components;
mod event_handler;
mod kunai;
mod memory_model;
mod proc_utils;
mod tui;
mod ui;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = tui::init()?;
    let mut kunai = Kunai::new();

    run_app(&mut terminal, &mut kunai);
    // dummy_runner(&mut kunai);

    Ok(tui::restore()?)
}

#[allow(dead_code)]
fn dummy_runner(kunai: &mut Kunai) {
    kunai.tasks.refresh_list();
    println!("{:#?}", kunai.tasks.task_list);
    println!("Hi!");
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, kunai: &mut Kunai) {
    kunai.tasks.refresh_list();
    loop {
        terminal.draw(|f| render_ui(f, kunai)).unwrap(); // TODO: Error handling

        // TODO: Background thread?
        if !handle_keypress(kunai) {
            break;
        }
    }
}
