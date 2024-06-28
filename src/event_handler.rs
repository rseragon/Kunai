use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::{
    kunai::Kunai,
    ui::{CurrentScreen, SubScreen},
};

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
                    CurrentScreen::TaskSelectionScreen => return handle_taskselection(kunai, key),
                    CurrentScreen::MemoryEditingScreen => return handle_memoryeditor(kunai, key),
                }
            }
        }
        Err(_) => return true,
    }

    true
}

fn handle_taskselection(kunai: &mut Kunai, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Up => kunai.tasks.decrement_index(),
        KeyCode::Down => kunai.tasks.increment_index(),
        KeyCode::Char(c) => {
            if kunai.tasks.name_search || kunai.tasks.pid_search {
                kunai.tasks.search_string.push(c);
                kunai.tasks.update_filtered_list();

                // Deselect idx as it might be out of range
                kunai.tasks.deselect_index();
            } else {
                match c {
                    'k' => kunai.tasks.decrement_index(),
                    'j' => kunai.tasks.increment_index(),
                    'r' => kunai.tasks.refresh_list(),
                    '/' => kunai.tasks.start_name_search(),
                    'g' => kunai.tasks.start_pid_search(),
                    'q' => return false,
                    _ => {}
                }
            }
        }
        KeyCode::Esc => {
            if kunai.tasks.name_search || kunai.tasks.pid_search {
                kunai.tasks.stop_search();
            } else {
                return false;
            }
        }
        KeyCode::Backspace => {
            if kunai.tasks.name_search || kunai.tasks.pid_search {
                kunai.tasks.search_string.pop();
                kunai.tasks.update_filtered_list();

                // Deselect idx as it might be out of range
                kunai.tasks.deselect_index();
            }
        }
        KeyCode::Enter if kunai.tasks.selected_task_idx.is_some() => {
            // TODO: Cloning bad?
            kunai
                .select_task(kunai.tasks.task_list[kunai.tasks.selected_task_idx.unwrap()].clone());
        }
        _ => {}
    }

    true
}

fn handle_memoryeditor(kunai: &mut Kunai, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Tab => {
            kunai.memedit.sub_screen = match kunai.memedit.sub_screen {
                SubScreen::MemorySearch => SubScreen::MemoryMaps,
                SubScreen::MemoryMaps => SubScreen::MemorySearch,
                SubScreen::ValueEditing => SubScreen::MemorySearch,
            }
        }
        KeyCode::Esc => match kunai.memedit.sub_screen {
            SubScreen::MemorySearch => kunai.current_screen = CurrentScreen::TaskSelectionScreen,
            SubScreen::MemoryMaps => kunai.memedit.sub_screen = SubScreen::MemorySearch,
            SubScreen::ValueEditing => kunai.memedit.sub_screen = SubScreen::MemorySearch,
        },
        KeyCode::Char(c) => match kunai.memedit.sub_screen {
            SubScreen::MemorySearch => kunai.memedit.search_string.push(c),
            SubScreen::MemoryMaps => {}
            SubScreen::ValueEditing => {}
        },
        KeyCode::Backspace => match kunai.memedit.sub_screen {
            SubScreen::MemorySearch => {
                kunai.memedit.search_string.pop();
            }
            SubScreen::MemoryMaps => {}
            SubScreen::ValueEditing => {}
        },
        KeyCode::Enter => match kunai.memedit.sub_screen {
            SubScreen::MemorySearch => {
                kunai.memedit.search_memory();
            }
            SubScreen::MemoryMaps => {
                // TODO: Enable memoroy search toggle
            }
            SubScreen::ValueEditing => {}
        },
        _ => {}
    }

    true
}
