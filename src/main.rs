use std::{error::Error, io::Stdout};

use event_handler::handle_keypress;
use kunai::Kunai;
use logging::initialize_logging;
use proc_utils::read_maps;
use ratatui::{backend::CrosstermBackend, Terminal};
use ui::render_ui;

use crate::{memory_model::search_mem, utils::num_to_hex};

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
    let mut kunai = Kunai::new();

    let mut terminal = tui::init()?;
    run_app(&mut terminal, &mut kunai);
    Ok(tui::restore()?)

    // dummy_runner(&mut kunai);
    // Ok(())
}

#[allow(dead_code)]
fn dummy_runner(kunai: &mut Kunai) {
    let pid = "11489".to_string();
    let maps = read_maps(&pid).unwrap();
    let search_string = "SomeString".to_string();

    for map in &maps {
        println!(
            "Reading: {} ({} - {}) [{}]",
            map.name,
            num_to_hex(map.start),
            num_to_hex(map.end),
            map.perms
        );
        let res = search_mem(&pid, &search_string, map).unwrap();
        println!("{:?}", res);
    }
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
