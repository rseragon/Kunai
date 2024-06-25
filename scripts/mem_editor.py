from io import BufferedRandom, TextIOWrapper
import sys
from dataclasses import dataclass
from pprint import pprint


def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)


@dataclass
class Mem:
    name: str
    start: str
    end: str
    perms: str


def mem_maps(maps_path):
    memory_maps = []
    try:
        with open(maps_path, "r") as maps_file:
            for line in maps_file:
                sline = line.split()
                mem_name = sline[-1] if len(sline) > 5 else f"[{sline[0]}]"
                addr = sline[0].split("-")
                if len(addr) != 2:
                    eprint("[!] Invalid memory:", line)
                    continue
                start = addr[0]
                end = addr[1]
                perms = sline[1]

                m = Mem(mem_name, start, end, perms)
                memory_maps.append(m)
    except Exception as e:
        eprint("[!] Failed to open maps: ", e)
        exit(-1)

    return memory_maps


def read_mem(mem_file: BufferedRandom, start: int, end: int):
    mem_file.seek(start)
    return mem_file.read(end - start)


def edit_mem(mem_file: BufferedRandom, mem: Mem, original: str, changed: str):
    if mem.perms.find("r") == -1:
        eprint("[!] No read perms for", mem.name)
        return
    if mem.perms.find("w") == -1:
        eprint("[!] No write perms for", mem.name)
        return
    start = int(mem.start, 16)
    end = int(mem.end, 16)
    m = read_mem(mem_file, start, end)

    idx = m.find(bytes(original, "ASCII"))
    if idx == -1:
        eprint("[!] String not found in", mem.name)
        return
    print("[*] String found at:", idx, "in", mem.name)

    mem_file.seek(start + idx)
    mem_file.write(bytes(changed, "ASCII"))
    print(f"[*] Changed {original} to {changed} in {mem.name}")


def main():
    if len(sys.argv) < 4:
        print(f"Usage: {sys.argv[0]} <pid> <originalString> <changedString>")
        exit(-1)

    pid = sys.argv[1]
    string_to_search = sys.argv[2]
    change_to = sys.argv[3]

    maps_path = f"/proc/{pid}/maps"

    maps = mem_maps(maps_path)
    # pprint(maps)

    mem_path = f"/proc/{pid}/mem"

    with open(mem_path, "rb+") as mem_file:
        for m in maps:
            edit_mem(mem_file, m, string_to_search, change_to)


if __name__ == "__main__":
    main()
