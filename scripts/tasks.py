import os
from pathlib import Path
from pprint import pprint
from typing import Tuple

# List all directories in /proc that are numbers
proc = "/proc/"

dirs = [
    proc + file
    for file in os.listdir(proc)
    if Path(proc + file).is_dir() and file.isdigit()
]

# PID: {name, state, cmdline}
info = {}


def read_cmdline(pid) -> str:
    with open(pid + "/cmdline", "r") as f:
        return f.read().replace("\x00", "")


def read_status(pid) -> Tuple[str, str]:
    name, state = "", ""
    with open(pid + "/status", "r") as f:
        lines = f.readlines()
        for line in lines:
            if line.find("Name:") != -1:
                name = line.split(":")[1].strip()
            elif line.find("State:") != -1:
                state = line.split(":")[1].strip()
    return name, state


for pid in dirs:
    cmdline = read_cmdline(pid)
    name, state = read_status(pid)

    info[pid] = [name, state, cmdline]

pprint(info)
