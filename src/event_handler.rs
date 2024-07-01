use core::panic;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

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
            SubScreen::MemorySearch => {
                if key.modifiers == KeyModifiers::CONTROL {
                    match c {
                        // TODO: Impl refresh search
                        'r' => kunai.memedit.search_memory(),
                        'm' => kunai.memedit.sub_screen = SubScreen::MemoryMaps,
                        'e' => kunai.memedit.sub_screen = SubScreen::ValueEditing,
                        _ => {}
                    }
                } else {
                    kunai.memedit.search_string.push(c)
                }
            }
            SubScreen::MemoryMaps => {}
            SubScreen::ValueEditing => {
                kunai.memedit.new_value.push(c);
            }
        },
        KeyCode::Backspace => match kunai.memedit.sub_screen {
            SubScreen::MemorySearch => {
                kunai.memedit.search_string.pop();
            }
            SubScreen::MemoryMaps => {}
            SubScreen::ValueEditing => {
                kunai.memedit.new_value.pop();
            }
        },
        KeyCode::Enter => match kunai.memedit.sub_screen {
            SubScreen::MemorySearch => {
                // reset search Table state
                kunai.memedit.search_table_state.select(None);
                kunai.memedit.search_memory();
            }
            SubScreen::MemoryMaps => {
                // TODO: Enable memoroy search toggle
            }
            SubScreen::ValueEditing => {}
        },
        KeyCode::Up => match kunai.memedit.sub_screen {
            SubScreen::MemoryMaps => {
                let mut curr_idx = kunai.memedit.map_table_state.selected().unwrap_or(0);
                if curr_idx > 0 {
                    curr_idx -= 1;
                } else {
                    curr_idx = kunai.memedit.task_mem.maps.len() - 1;
                }
                kunai.memedit.map_table_state.select(Some(curr_idx));
            }
            SubScreen::MemorySearch => {
                let mut curr_idx = kunai.memedit.search_table_state.selected().unwrap_or(0);
                if curr_idx > 0 {
                    curr_idx -= 1;
                } else {
                    curr_idx = kunai.memedit.search_list.len() - 1;
                }
                kunai.memedit.search_table_state.select(Some(curr_idx));
            }
            SubScreen::ValueEditing => {}
        },
        KeyCode::Down => match kunai.memedit.sub_screen {
            SubScreen::MemoryMaps => {
                let mut curr_idx = kunai.memedit.map_table_state.selected().unwrap_or(0);
                if curr_idx >= kunai.memedit.task_mem.maps.len() - 1 {
                    curr_idx = 0;
                } else {
                    curr_idx += 1;
                }
                kunai.memedit.map_table_state.select(Some(curr_idx));
            }
            SubScreen::MemorySearch => {
                let mut curr_idx = kunai.memedit.search_table_state.selected().unwrap_or(0);
                if curr_idx >= kunai.memedit.search_list.len() - 1 {
                    curr_idx = 0;
                } else {
                    curr_idx += 1;
                }
                kunai.memedit.search_table_state.select(Some(curr_idx));
            }
            SubScreen::ValueEditing => {}
        },
        _ => {}
    }

    true
}
