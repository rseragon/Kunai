use std::usize;

use ratatui::widgets::TableState;

use crate::{proc_utils::get_tasks, ui::CurrentScreen};

#[derive(Debug)]
pub struct Task {
    pub pid: String,
    pub name: String,
    pub state: String,
    pub cmdline: String,
}

#[derive(Debug)]
pub struct TaskSelection {
    pub task_list: Vec<Task>,
    pub selected_task_idx: Option<usize>,

    pub table_state: TableState,

    // Searching
    pub name_search: bool,
    pub pid_search: bool,
    pub search_string: String,
}

#[derive(Debug)]
pub struct Kunai {
    pub tasks: TaskSelection,
    pub current_screen: CurrentScreen,
}

impl Kunai {
    pub fn new() -> Kunai {
        Kunai {
            tasks: TaskSelection {
                task_list: Vec::new(),
                selected_task_idx: None,
                table_state: TableState::default(),
                name_search: false,
                pid_search: false,
                search_string: String::new(),
            },
            current_screen: CurrentScreen::TaskSelection, // The initial screen
        }
    }
}

impl TaskSelection {
    pub fn refresh_list(&mut self) {
        match get_tasks() {
            Ok(tasks) => self.task_list = tasks,
            Err(_) => self.task_list = Vec::new(),
        };
    }

    pub fn increment_index(&mut self) {
        if self.selected_task_idx.is_none() {
            self.selected_task_idx = Some(0);
        }
        self.selected_task_idx = Some((self.selected_task_idx.unwrap() + 1) % self.task_list.len());

        self.table_state.select(self.selected_task_idx);
    }

    pub fn decrement_index(&mut self) {
        if self.selected_task_idx.is_none() {
            self.selected_task_idx = Some(0);
        }
        let idx = self.selected_task_idx.unwrap();

        if idx == 0 {
            self.selected_task_idx = Some(self.task_list.len() - 1);
        } else {
            self.selected_task_idx = Some((idx - 1) % self.task_list.len());
        }

        self.table_state.select(self.selected_task_idx);
    }

    pub fn deselect_index(&mut self) {
        self.selected_task_idx = None;
        self.table_state.select(self.selected_task_idx);
    }

    pub fn stop_search(&mut self) {
        self.pid_search = false;
        self.name_search = false;
    }

    pub fn start_pid_search(&mut self) {
        self.pid_search = true;
    }

    pub fn start_name_search(&mut self) {
        self.name_search = true;
    }

    pub fn select_task(&mut self) {
        // TODO: Update screen && send selected task info to next screen
    }
}

impl Task {
    pub fn new() -> Task {
        Task {
            pid: String::new(),
            name: String::new(),
            state: String::new(),
            cmdline: String::new(),
        }
    }
}
