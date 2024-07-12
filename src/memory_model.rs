use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
    usize,
};

use memchr::memmem;

use crate::{proc_utils::read_maps, trace_dbg, utils::bytes_to_string};

/// TODO: Do I require this struct?
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
    pub start: usize,
    pub end: usize,
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

#[derive(Debug, Clone)]
pub struct SearchLocation {
    pub start: usize,
    pub end: usize,
    pub value: String,
    pub prev_value: String,
    pub mem_info: MemoryMap,
    // TODO: prev value
}

impl SearchLocation {
    pub fn new() -> SearchLocation {
        SearchLocation {
            start: 0,
            end: 0,
            value: String::new(),
            prev_value: String::new(),
            mem_info: MemoryMap::new(), // TODO: This is bad
        }
    }
}

pub fn read_mem(pid: &String, start: usize, end: usize) -> io::Result<Vec<u8>> {
    let mem_file = "/proc/".to_string() + pid + "/mem";
    let mut mem = match File::open(mem_file) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    trace_dbg!(start);
    trace_dbg!(end);

    let read_len = start.abs_diff(end); // |start - end| (Mathematical expression)

    trace_dbg!(read_len);

    let mut mem_buf = vec![0u8; read_len];

    mem.seek(SeekFrom::Start(start as u64))?;

    mem.read_exact(&mut mem_buf)?;

    Ok(mem_buf)
}

/// Searches a single map of memory
pub fn search_mem(
    pid: &String,
    search_string: &String,
    map: &MemoryMap,
) -> io::Result<Vec<SearchLocation>> {
    let mut locs = Vec::new();

    // open mem file
    let mem_file = "/proc/".to_string() + pid + "/mem";
    let mut mem = match File::open(mem_file) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    // Convert the search string to bytes
    let search_bytes = search_string.as_bytes();

    let mem_start = map.start;
    let mem_end = map.end;

    mem.seek(SeekFrom::Start(mem_start as u64))?;

    let mem_size = mem_end - mem_start;
    trace_dbg!(mem_size);

    let mut mem_buf = vec![0u8; mem_size];

    mem.read_exact(&mut mem_buf)?;

    let it = memmem::find_iter(&mem_buf, &search_bytes);

    for occurance in it {
        // println!("Found occurance in {} at {}", map.name, occurance);
        let mut loc = SearchLocation::new();

        loc.start = map.start + occurance;
        loc.end = map.start + occurance + search_bytes.len();
        trace_dbg!(loc.end);
        loc.mem_info = map.clone();

        match mem.seek(SeekFrom::Start(loc.start as u64)) {
            Ok(_) => {}
            Err(e) => {
                trace_dbg!(e);
                continue;
            }
        }
        let mut value = vec![0u8; search_bytes.len()];
        match mem.read_exact(&mut value) {
            Ok(_) => {}
            Err(e) => {
                trace_dbg!(e);
                continue;
            }
        }

        loc.value = bytes_to_string(value);

        trace_dbg!(&loc);

        locs.push(loc);
    }

    Ok(locs)
}
