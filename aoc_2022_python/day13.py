from aoc import input_fpath_for_day
import json


def parse_line(line):
    """
    >>> parse_line("[1,1,3,1,1]")
    [1, 1, 3, 1, 1]
    >>> parse_line("[]")
    []
    """
    parsed = eval(line.strip())
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

    zipped = list(zip(*packet_pair))
    diff = 0
    ix = 0
    for _ix in range(len(zipped)):
        ix = _ix
        left, right = zipped[_ix]
        # If both values are integers, the lower integer should come first (be left)
        if isinstance(left, int) and isinstance(right, int):
            if left > right:
                return False
            # Keep track of the difference between the left and right values
            diff += right - left
        elif isinstance(left, list) and isinstance(right, list):
            if not correct_order((left, right)):
                return False
        else:
            left = left if isinstance(left, list) else [left]
            right = right if isinstance(right, list) else [right]
            if not correct_order((left, right)):
                return False
    left, right = packet_pair
    # If the difference between left and right is greater than 0, the order is correct
    # If the left packet is shorter or equal in length to the right packet, the order is correct
    return diff > 0 or len(left[ix:]) <= len(right[ix:])


def part_1(puzzle_input):
    packet_pairs = packet_pairs_from_puzzle_input(puzzle_input)
    return sum(ix + 1 for ix, pair in enumerate(packet_pairs) if correct_order(pair))


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
        #  content = f.read()
        print("Day 13")
        print("Part 1:", part_1(content))
        print("Part 2:", part_2(content))
