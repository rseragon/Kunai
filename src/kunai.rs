use std::{
    fs::File,
    io::{Seek, SeekFrom, Write},
    os::unix::{fs::FileExt, thread},
    usize,
};

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
    pub selected_value: Option<SearchLocation>,
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

    /// Select task from `filtered_task_list` if list is filtered
    /// else take from `task_list`
    pub fn select_task(&mut self, index: usize) {
        let task = match &self.tasks.filtered_task_list {
            Some(list) => list[index].clone(),
            None => {
                // TODO: Show UI error here
                return;
            }
        };

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
        let list_len = match &self.filtered_task_list {
            Some(l) => l.len(),
            None => self.task_list.len(),
        };

        let idx = match self.table_state.selected() {
            Some(i) => (i + 1) % list_len,
            None => 0,
        };

        self.table_state.select(Some(idx));
    }

    pub fn decrement_index(&mut self) {
        let list_len = match &self.filtered_task_list {
            Some(l) => l.len(),
            None => self.task_list.len(),
        };

        let idx = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    i - 1
                } else {
                    (i - 1) % list_len
                }
            }
            None => list_len - 1,
        };

        self.table_state.select(Some(idx));
    }

    pub fn deselect_index(&mut self) {
        self.table_state.select(None);
    }

    pub fn stop_search(&mut self) {
        self.pid_search = false;
        self.name_search = false;
        self.filtered_task_list = None;
        self.table_state.select(None);
    }

    pub fn start_pid_search(&mut self) {
        self.pid_search = true;
        self.table_state.select(None);
    }

    pub fn start_name_search(&mut self) {
        self.name_search = true;
        self.table_state.select(None);
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

        self.table_state.select(None);
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
            selected_value: None,
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

    pub fn edit_memory(&mut self) {
        let location = match &self.selected_value {
            Some(l) => l,
            None => {
                self.ui_msg = Some("Invalid memory address".to_string());
                return;
            }
        };

        let mem_file = "/proc/".to_string() + &self.task.pid + "/mem";
        let mut mem = match File::options().read(true).write(true).open(mem_file) {
            Ok(f) => f,
            Err(e) => {
                self.ui_msg = Some("Failed to open mem file".to_string());
                trace_dbg!(e);
                return;
            }
        };

        let seek = location.start;
        let new_val = self.new_value.as_bytes();

        match mem.seek(SeekFrom::Start(seek as u64)) {
            Ok(_) => {}
            Err(e) => {
                self.ui_msg = Some("Failed to seek mem file".to_string());
                trace_dbg!(e);
                return;
            }
        }

        match mem.write_all(new_val) {
            Ok(_) => {
                self.ui_msg = Some("Written!".to_string());
            }
            Err(e) => {
                self.ui_msg = Some("Failed to write at memory".to_string());
                trace_dbg!(e);
            }
        };
    }
}
