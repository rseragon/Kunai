use std::{error::Error, io::Stdout};

use event_handler::handle_keypress;
use kunai::Kunai;
use logging::initialize_logging;
use proc_utils::read_maps;
use ratatui::{backend::CrosstermBackend, Terminal};
use ui::render_ui;

mod components;
mod event_handler;
mod kunai;
mod logging;
mod memory_model;
mod proc_utils;
mod tui;
mod ui;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    initialize_logging()?;

    let mut terminal = tui::init()?;
    let mut kunai = Kunai::new();

    run_app(&mut terminal, &mut kunai);
    // dummy_runner(&mut kunai);

    Ok(tui::restore()?)
    // Ok(())
}

#[allow(dead_code)]
fn dummy_runner(kunai: &mut Kunai) {
    let pid = "1836".to_string();
    let maps = read_maps(&pid).unwrap();
    println!("{:?}", maps);
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
