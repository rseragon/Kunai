use std::{
    fs::{self},
    io,
};

use crate::{kunai::Task, utils::is_numeric};

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
