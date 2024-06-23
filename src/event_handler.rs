use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::{kunai::Kunai, ui::CurrentScreen};

// Returns a boolean to justify further processing of events
pub fn handle_keypress(kunai: &mut Kunai) -> bool {
    match event::read() {
        Ok(e) => {
            if let Event::Key(key) = e {
                if key.kind == event::KeyEventKind::Release {
                    // Skip events that are not KeyEventKind::Press
                    return true;
                }

                match kunai.current_screen {
                    CurrentScreen::TaskSelection => return handle_taskselection(kunai, key),
                }
            }
        }
        Err(_) => return true,
    }

    true
}

fn handle_taskselection(kunai: &mut Kunai, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            kunai.tasks.decrement_index();
            kunai
                .tasks
                .table_state
                .select(kunai.tasks.selected_task_idx);
        }

        KeyCode::Down | KeyCode::Char('j') => {
            kunai.tasks.increment_index();
            kunai
                .tasks
                .table_state
                .select(kunai.tasks.selected_task_idx);
        }
        KeyCode::Char('q') => return false,
        _ => {}
    }

    true
}
