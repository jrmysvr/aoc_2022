import pathlib
import re


def get_crate_indexes(n_crates):
    """
    >>> get_crate_indexes(3)
    [1, 5, 9]
    >>> get_crate_indexes(4)
    [1, 5, 9, 13]
    >>> get_crate_indexes(5)
    [1, 5, 9, 13, 17]
    """
    step = 4
    return list(range(1, n_crates * step, step))


def parse_crates_content(crates_content):
    crates_lines = re.sub(r"[\[\]]", " ", crates_content).splitlines()
    crates_lines = crates_lines[::-1]

    n_crates = int(crates_lines[0].strip()[-1])
    crates = [list() for _ in range(n_crates)]
    crate_indexes = get_crate_indexes(n_crates)
    for _line in crates_lines[1:]:
        for ix, crate in enumerate(_line):
            if ix in crate_indexes and crate.isalpha():
                crate_ix = crate_indexes.index(ix)
                crates[crate_ix].append(crate)

    return crates


def parse_procedure_content(procedure_content):
    procedure_lines = procedure_content.splitlines()
    return [
        (
            int(re.search(r"(?<=move )[0-9]*", line).group()),
            int(re.search(r"((?<=from )[0-9]*)", line).group()) - 1,
            int(re.search(r"((?<= to )[0-9]*)", line).group()) - 1,
        )
        for line in procedure_lines
    ]


def parse_input(input_content):
    crates_content, procedure_content = input_content.strip("\n").split("\n\n")
    crates = parse_crates_content(crates_content)
    procedure = parse_procedure_content(procedure_content)
    return (crates, procedure)


def part_1(puzzle_input):
    crates, procedure = parse_input(puzzle_input)
    for (n_crates, from_, to_) in procedure:
        for _ in range(n_crates):
            crates[to_].append(crates[from_].pop())

    return "".join(crate[-1] for crate in crates)


def part_2(puzzle_input):
    crates, procedure = parse_input(puzzle_input)
    for (n_crates, from_, to_) in procedure:
        crates_to_move = crates[from_][-n_crates:]
        crates[from_] = crates[from_][:-n_crates]
        crates[to_] = crates[to_] + crates_to_move

    return "".join(crate[-1] for crate in crates)


if __name__ == "__main__":
    input_fpath = pathlib.Path(__file__).parent / ".." / "inputs" / "day5.txt"
    with input_fpath.open() as f:
        content = """
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"""
        content = f.read()
        print("Day 5")
        print("Part 1:", part_1(content))
        print("Part 2:", part_2(content))
