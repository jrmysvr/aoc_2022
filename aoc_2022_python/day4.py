import pathlib


def string_to_range(string):
    start, stop = tuple(map(int, string.split("-")))
    return range(start, stop + 1)


def parse_line_to_ranges(line):
    fst, snd = tuple(map(string_to_range, line.split(",")))
    return (fst, snd)


def overlapping_ranges(ranges):
    fst, snd = tuple(map(set, ranges))
    return fst.issuperset(snd) or snd.issuperset(fst)


def partial_overlapping_ranges(ranges):
    fst, snd = tuple(map(set, ranges))
    return fst.intersection(snd) or snd.intersection(fst)


def part_1(input_lines):
    return len(list(filter(overlapping_ranges, map(parse_line_to_ranges, input_lines))))


def part_2(input_lines):
    return len(list(filter(partial_overlapping_ranges, map(parse_line_to_ranges, input_lines))))


if __name__ == "__main__":
    input_fpath = pathlib.Path(__file__).parent / ".." / "inputs" / "day4.txt"
    with input_fpath.open() as f:
        content = f.read().strip().splitlines()
        print("Day 4")
        print("Part 1:", part_1(content))
        print("Part 2:", part_2(content))
