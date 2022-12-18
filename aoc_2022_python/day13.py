from aoc import input_fpath_for_day
from itertools import zip_longest


def _parse_line(line):
    #  parsed = eval(line.strip())
    parsed = list()
    # Skip the first bracket
    ix = 1
    while ix < len(line):
        ch = line[ix]
        if ch.isnumeric():
            parsed.append(int(ch))
            ix += 1
        elif ch == "[":
            _ix, _parsed = _parse_line(line[ix:])
            parsed.append(_parsed)
            ix += _ix
        elif ch == "]":
            ix += 1
            return ix, parsed
        else:
            ix += 1

    return ix, parsed


def parse_line(line):
    """
    >>> parse_line("[1,1,3,1,1]")
    [1, 1, 3, 1, 1]
    >>> parse_line("[]")
    []
    >>> parse_line("[[]]")
    [[]]
    >>> parse_line("[1,[2,[3,[[[4,5]]],6]]]")
    [1, [2, [3, [[[4, 5]]], 6]]]
    >>> parse_line("[[[],[],[[]]]]")
    [[[], [], [[]]]]
    """
    _, parsed = _parse_line(line)
    return parsed


def packet_pair_from(lines):
    _left, _right = lines.strip().split("\n")
    left = parse_line(_left)
    right = parse_line(_right)
    return (left, right)


def packet_pairs_from_puzzle_input(puzzle_input):
    return list(map(packet_pair_from, puzzle_input.strip().split("\n\n")))


def correct_order(packet_pair):
    """
    >>> correct_order(([1, 1, 3, 1, 1], [1, 1, 5, 1, 1]))
    True
    >>> correct_order(([[1], [2, 3, 4]], [[1], 4]))
    True
    >>> correct_order(([9], [[8, 7, 6]]))
    False
    >>> correct_order(([[4, 4], 4, 4], [[4, 4], 4, 4, 4]))
    True
    >>> correct_order(([7, 7, 7, 7], [7, 7, 7]))
    False
    >>> correct_order(([7, 7, 7, 7], [7, 7, 7, 7]))
    True
    >>> correct_order(([7, 7, 7], [7, 7, 7, 7]))
    True
    >>> correct_order(([], [3]))
    True
    >>> correct_order(([[[]]], [[]]))
    False
    >>> correct_order(([1,[2,[3,[4,[5,6,7]]]],8,9], [1,[2,[3,[4,[5,6,0]]]],8,9]))
    False
    >>> correct_order(([2], [[[10], 4]]))
    True
    >>> correct_order(([10], [[]]))
    False
    >>> correct_order(([[10]], [10, [[[[[10]]]]]]))
    True
    >>> correct_order(([[]], [[[[[[10]]]]]]))
    True
    >>> correct_order(([[1], [2], [[3]]], [[1], 2, [3]]))
    True
    """
    return all(map(compare, zip(*packet_pair)))
    #  return all(map(compare, zip_longest(*packet_pair)))


def compare(ab):
    """
    >>> compare(([2, 3, 4], 4))
    True
    """
    a, b = ab
    if not a:
        return True
    if not b:
        return False
    if isinstance(a, list) and isinstance(b, list):
        if not any(a):
            return True
        if not any(b):
            return False
        return all(map(compare, zip_longest(a, b, fillvalue=-1)))

    if isinstance(a, int) and isinstance(b, int):
        return a <= b

    a = a if isinstance(a, list) else [a]
    b = b if isinstance(b, list) else [b]
    return compare(*zip(*(a, b)))


def part_1(puzzle_input):
    packet_pairs = packet_pairs_from_puzzle_input(puzzle_input)
    return sum(
        ix + 1 for ix, pair in enumerate(packet_pairs) if correct_order(pair)
    )


def part_2(puzzle_input):
    pass


if __name__ == "__main__":
    with input_fpath_for_day(13).open() as f:
        content = """\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"""
        content = f.read()
        print("Day 13")
        print("Part 1:", part_1(content))
        print("Part 2:", part_2(content))
