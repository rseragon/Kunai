#[derive(Debug)]
pub struct TaskMemory {
    pub map: Vec<MemoryMap>,
}

impl TaskMemory {
    pub fn new() -> TaskMemory {
        TaskMemory { map: Vec::new() }
    }
}

/// Example
/// 7ffffe15a000-7ffffe17c000   rw-p   00000000 00:00 0       [stack]
///  start         end         perms                           name        
#[derive(Debug)]
pub struct MemoryMap {
    pub start: String,
    pub end: String,
    pub perms: String,
    pub name: String,

    // UI stuff
    pub should_search: bool, // Deafult true
}

impl MemoryMap {
    pub fn new() -> MemoryMap {
        MemoryMap {
            start: String::new(),
            end: String::new(),
            perms: String::new(),
            name: String::new(),
            should_search: true,
        }
    }
}
