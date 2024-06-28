use crate::{kunai::Task, proc_utils::read_maps};

#[derive(Debug)]
pub struct TaskMemory {
    pub maps: Vec<MemoryMap>,
}

/// Utilizes lazy loading... (Don't wanna read info until called for)
impl TaskMemory {
    pub fn new() -> TaskMemory {
        TaskMemory { maps: Vec::new() }
    }

    pub fn populate_info(&mut self, pid: &String) {
        self.maps = match read_maps(pid) {
            Ok(m) => m,
            Err(_) => Vec::new(), // TODO: Error handling here!
        };
    }
}

/// Example
/// 7ffffe15a000-7ffffe17c000   rw-p   00000000 00:00 0       [stack]
///  start         end         perms                           name        
///  Start, end are converted into usize
#[derive(Debug, Clone)]
pub struct MemoryMap {
    pub start: i64,
    pub end: i64,
    pub perms: String,
    pub name: String,

    // UI stuff
    pub should_search: bool, // Deafult true
}

impl MemoryMap {
    pub fn new() -> MemoryMap {
        MemoryMap {
            start: 0,
            end: 0,
            perms: String::new(),
            name: String::new(),
            should_search: true,
        }
    }
}

#[derive(Debug)]
pub struct SearchLocation {
    pub start: i64,
    pub end: i64,
    pub value: String,
    pub mem_info: MemoryMap,
    // TODO: prev value
}

impl SearchLocation {
    pub fn new() -> SearchLocation {
        SearchLocation {
            start: 0,
            end: 0,
            value: String::new(),
            mem_info: MemoryMap::new(), // TODO: This is bad
        }
    }
}
