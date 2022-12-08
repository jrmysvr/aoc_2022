from aoc import input_fpath_for_day
import json


def parse_input(puzzle_input):
    for line in puzzle_input.splitlines():
        yield line.split(" ")


def calc_directory_sizes(fs: dict, cur_name: str):
    sizes = dict()
    cur_size = 0
    for (name, ftype) in fs.items():
        if isinstance(ftype, dict) and name != "..":
            sizes[name] = calc_directory_sizes(ftype, name)
            cur_size += sizes[name][name]
        else:
            if ftype and name != ".." and name != cur_name:
                cur_size += ftype[-1]  # (name, file size)

    sizes[cur_name] = cur_size
    return sizes


def print_fs(fs: dict, space=""):
    for (k, v) in fs.items():
        if isinstance(v, dict) and k != "..":
            print(space, "-", k, "(dir)")
            print_fs(v, space + " ")
        else:
            if k != "..":
                print(space, "-", k, f"(file, size={v[1]})")


def filter_sizes_under_value(sizes, value):
    def lte(a, b):
        return a <= b

    return __filter_sizes_by_value(sizes, value, lte)


def filter_sizes_over_value(sizes, value):
    def gte(a, b):
        return a >= b

    return __filter_sizes_by_value(sizes, value, gte)


def __filter_sizes_by_value(sizes, value, cmp):
    filtered = list()
    for (k, v) in sizes.items():
        if isinstance(v, dict):
            filtered += __filter_sizes_by_value(v, value, cmp)
        else:
            if cmp(v, value):
                filtered.append(v)

    return filtered


def file_system_from_puzzle_input(puzzle_input):
    insts = parse_input(puzzle_input)
    parent = None
    fs = {"/": {"..": parent}}
    root = fs["/"]
    cwd = root
    for fst, *rest in insts:
        if fst == "$":
            cmd = rest[0]
            if cmd == "cd":
                dirname = rest[-1]
                if dirname == "/":
                    continue
                parent = cwd
                cwd = cwd[dirname]
            elif fst == "ls":
                continue
        elif fst == "dir":
            dirname = rest[-1]
            cwd[dirname] = {"..": cwd}
        else:  # New File
            size = int(fst)
            fname = rest[-1]
            cwd[fname] = (fname, size)

    return fs


def part_1(puzzle_input):
    fs = file_system_from_puzzle_input(puzzle_input)
    # print_fs(fs)
    sizes = calc_directory_sizes(fs["/"], "/")
    return sum(filter_sizes_under_value(sizes, 100000))


def part_2(puzzle_input):
    fs = file_system_from_puzzle_input(puzzle_input)
    sizes = calc_directory_sizes(fs["/"], "/")

    total_used = sizes["/"]
    disk_size = 70_000_000
    desired_free_space = 30_000_000
    unused_space = disk_size - total_used
    space_to_free = desired_free_space - unused_space
    dirs_to_delete = filter_sizes_over_value(sizes, space_to_free)
    return min(dirs_to_delete)


if __name__ == "__main__":
    with input_fpath_for_day(7).open() as f:
        content = """\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"""
        content = f.read()
        print("Day 7")
        print("Part 1:", part_1(content))
        print("Part 2:", part_2(content))
