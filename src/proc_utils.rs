use std::{fs, io, usize};

use crate::{kunai::Task, memory_model::MemoryMap, utils::is_numeric};

pub fn get_tasks() -> Result<Vec<Task>, io::Error> {
    let mut task_list = Vec::new();
    let pids = get_pids()?;

    for pid in pids {
        match get_task_info(&pid) {
            Ok(info) => task_list.push(info),
            Err(e) => {
                // println!("{:?}", e);
            }
        };
    }

    Ok(task_list)
}

pub fn read_maps(pid: &String) -> Result<Vec<MemoryMap>, io::Error> {
    let mut maps = Vec::new();

    let maps_file = "/proc/".to_string() + pid + "/maps";

    let maps_str = fs::read_to_string(maps_file)?;

    for line in maps_str.lines() {
        let mut mm = MemoryMap::new();
        let mut sline = line.split_ascii_whitespace();

        // The first part contains `start-end` mem
        let mut startend = sline.next();
        let mut memsplit = match startend.take() {
            Some(s) => s.split('-'),
            None => continue,
        };

        let start = match memsplit.next() {
            Some(s) => s,
            None => continue,
        };

        let end = match memsplit.next() {
            Some(s) => s,
            None => continue,
        };

        mm.start = match i64::from_str_radix(start, 16) {
            Ok(n) => n,
            Err(_) => continue,
        };
        mm.end = match i64::from_str_radix(end, 16) {
            Ok(n) => n,
            Err(_) => continue,
        };

        // Read perms
        let perms = match sline.next() {
            Some(s) => s,
            None => continue,
        };
        mm.perms = perms.to_string();

        // Skip the next 3
        sline.next();
        sline.next();
        sline.next();

        let mem_name = match sline.next() {
            Some(s) => s,
            None => "-", // This can be None
        };
        mm.name = mem_name.to_string();

        maps.push(mm);
    }

    Ok(maps)
}

fn get_pids() -> Result<Vec<String>, io::Error> {
    let mut pids = Vec::new();

    let paths = fs::read_dir("/proc/")?;

    for path in paths {
        let f = match path {
            Ok(file) => file,
            Err(_) => continue,
        };

        let pid = match f.file_name().into_string() {
            Ok(s) => s,
            Err(_) => continue,
        };
        if let Ok(file_type) = f.file_type() {
            if file_type.is_dir() && is_numeric(&pid) {
                pids.push(pid);
            }
        }
    }

    Ok(pids)
}

pub fn get_task_info(pid: &String) -> Result<Task, io::Error> {
    let mut task = Task::new();

    task.pid = String::from(pid);
    task.cmdline = get_cmdline(pid);

    let (name, state) = match get_pid_status(pid) {
        Ok((n, s)) => (n, s),
        Err(e) => return Err(e),
    };

    task.name = name;
    task.state = state;

    Ok(task)
}

fn get_pid_status(pid: &String) -> Result<(String, String), io::Error> {
    let status_file = "/proc/".to_string() + pid + "/status";
    let mut state = String::new();
    let mut name = String::new();

    let status = match fs::read_to_string(status_file) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    for line in status.lines() {
        if line.contains("Name:") {
            let mut split = line.split(":\t");
            split.next(); // Skip the first part
            name = match split.next() {
                Some(s) => String::from(s),
                None => String::new(),
            };
        }
        if line.contains("State:") {
            let mut split = line.split(":\t");
            split.next(); // Skip the first part
            state = match split.next() {
                Some(s) => String::from(s),
                None => String::new(),
            };
        }
    }

    Ok((name, state))
}

fn get_cmdline(pid: &String) -> String {
    let cmdline_file = "/proc/".to_string() + pid + "/cmdline";

    match fs::read_to_string(cmdline_file) {
        Ok(s) => s.replace('\0', " ").trim().to_owned(), // For some reason, the file contains \0
        Err(_) => String::new(),
    }
}
