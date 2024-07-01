use std::{os::unix::thread, usize};

use ratatui::widgets::TableState;

use crate::{
    memory_model::{search_mem, SearchLocation, TaskMemory},
    proc_utils::get_tasks,
    trace_dbg,
    ui::{CurrentScreen, SubScreen},
};

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
    pub filtered_task_list: Option<Vec<Task>>,
    pub ui_msg: Option<String>,
}

#[derive(Debug)]
pub struct MemoryEditor {
    pub task: Task,
    pub task_mem: TaskMemory,
    pub search_string: String,
    pub search_list: Vec<SearchLocation>,

    // UI Sfuff
    pub sub_screen: SubScreen,
    pub map_table_state: TableState,
    pub search_table_state: TableState,
    pub ui_msg: Option<String>,

    // Value editing shit
    pub selected_value: String,
    pub new_value: String,
}

#[derive(Debug)]
pub struct Kunai {
    pub tasks: TaskSelection,
    pub memedit: MemoryEditor,
    pub current_screen: CurrentScreen,
    pub ui_msg: Option<String>,
}

impl Kunai {
    pub fn new() -> Kunai {
        Kunai {
            tasks: TaskSelection::new(),
            memedit: MemoryEditor::new(),
            current_screen: CurrentScreen::TaskSelectionScreen, // The initial screen
            ui_msg: None,
        }
    }

    pub fn select_task(&mut self, task: Task) {
        self.memedit.task = task.clone();
        self.memedit.task_mem = TaskMemory::new();
        self.memedit.task_mem.populate_info(&task.pid);

        self.current_screen = CurrentScreen::MemoryEditingScreen;
    }
}

impl TaskSelection {
    pub fn new() -> TaskSelection {
        TaskSelection {
            task_list: Vec::new(),
            selected_task_idx: None,
            table_state: TableState::default(),
            name_search: false,
            pid_search: false,
            search_string: String::new(),
            filtered_task_list: None,
            ui_msg: None,
        }
    }

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
        let list_len = match &self.filtered_task_list {
            Some(l) => l.len(),
            None => self.task_list.len(),
        };

        self.selected_task_idx = Some((self.selected_task_idx.unwrap() + 1) % list_len);

        self.table_state.select(self.selected_task_idx);
    }

    pub fn decrement_index(&mut self) {
        if self.selected_task_idx.is_none() {
            self.selected_task_idx = Some(0);
        }

        let idx = self.selected_task_idx.unwrap();

        let list_len = match &self.filtered_task_list {
            Some(l) => l.len(),
            None => self.task_list.len(),
        };

        if idx == 0 {
            self.selected_task_idx = Some(list_len - 1);
        } else {
            self.selected_task_idx = Some((idx - 1) % list_len);
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
        self.filtered_task_list = None;
    }

    pub fn start_pid_search(&mut self) {
        self.pid_search = true;
    }

    pub fn start_name_search(&mut self) {
        self.name_search = true;
    }

    pub fn update_filtered_list(&mut self) {
        if self.search_string.is_empty() {
            self.filtered_task_list = None;
        }

        let mut filtered_list: Vec<Task> = Vec::new();

        if self.name_search {
            for t in &self.task_list {
                if t.name.contains(&self.search_string) {
                    filtered_list.push(t.clone());
                }
            }
        } else if self.pid_search {
            for t in &self.task_list {
                if t.pid.contains(&self.search_string) {
                    filtered_list.push(t.clone());
                }
            }
        }

        self.filtered_task_list = Some(filtered_list);
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

    pub fn clone(&self) -> Task {
        Task {
            pid: self.pid.clone(),
            name: self.name.clone(),
            state: self.state.clone(),
            cmdline: self.cmdline.clone(),
        }
    }
}

impl MemoryEditor {
    pub fn new() -> MemoryEditor {
        MemoryEditor {
            task: Task::new(),
            task_mem: TaskMemory::new(),
            sub_screen: SubScreen::MemorySearch,
            map_table_state: TableState::new(),
            search_table_state: TableState::new(),
            search_string: String::new(),
            search_list: Vec::new(),
            ui_msg: None,
            selected_value: String::new(),
            new_value: String::new(),
        }
    }

    pub fn search_memory(&mut self) {
        // TODO: This message isn't visible
        self.ui_msg = Some(format!("Searching: {}", self.search_string));
        let mut locations = Vec::new();

        let pid = &self.task.pid;
        let search_string = &self.search_string;

        for map in &self.task_mem.maps {
            let locs = match search_mem(pid, search_string, map) {
                Ok(l) => l,
                Err(e) => {
                    trace_dbg!(e);
                    continue;
                }
            };

            locations.extend(locs);
        }
        std::thread::sleep(std::time::Duration::from_secs(5));

        self.ui_msg = Some(format!("Found {} occurances!", locations.len()));
        self.search_list = locations;
    }

    pub fn edit_memory(&mut self, location: SearchLocation, new_value: String) {}
}
