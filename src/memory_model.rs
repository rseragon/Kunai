use core::panic;
use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
};

use memchr::memmem;

use crate::{kunai::Task, proc_utils::read_maps, trace_dbg, utils::bytes_to_string};

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

pub fn read_mem(pid: &String, start: i64, end: i64) -> io::Result<Vec<u8>> {
    let mem_file = "/proc/".to_string() + pid + "/mem";
    let mut mem = match File::open(mem_file) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let read_len = end - start;
    let mut mem_buf = vec![0u8; read_len as usize];

    mem.seek(SeekFrom::Current(start))?;

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

    mem.seek(SeekFrom::Current(mem_start))?;

    let mem_size = mem_end - mem_start;

    let mut mem_buf = vec![0u8; mem_size as usize];

    mem.read_exact(&mut mem_buf)?;

    let it = memmem::find_iter(&mem_buf, &search_bytes);

    for occurance in it {
        // println!("Found occurance in {} at {}", map.name, occurance);
        let mut loc = SearchLocation::new();

        loc.start = occurance as i64;
        loc.end = (occurance + search_bytes.len()) as i64;
        loc.mem_info = map.clone();
        // TODO: Read value at loc

        match mem.seek(SeekFrom::Start((map.start + loc.start) as u64)) {
            Ok(_) => {}
            Err(e) => {
                // println!("{e}");
                continue;
            }
        }

        let mut value = vec![0u8; (loc.end - loc.start) as usize];

        match mem.read_exact(&mut value) {
            Ok(_) => {}
            Err(e) => {
                // println!("{e}");
                panic!();
            }
        }

        loc.value = bytes_to_string(value);

        trace_dbg!(&loc);

        locs.push(loc);
    }

    Ok(locs)
}
