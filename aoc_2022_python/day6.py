from aoc import input_fpath_for_day


def find_marker_by_length(buffer, marker_length):
    """
    >>> find_start_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
    ('jpqm', 7)
    """

    for ix in range(marker_length - 1, len(buffer)):
        marker = buffer[ix - marker_length : ix]
        if len(set(marker)) == marker_length:
            return (str(marker), ix)

    raise Exception("No marker found!")
    # # Alternative approach
    #  marker_ix = next(
    #  ix
    #  for ix in range(marker_length - 1, len(buffer))
    #  if len(set(buffer[ix - marker_length : ix])) == marker_length
    #  )

    #  return (buffer[marker_ix - marker_length : marker_ix], marker_ix)


def part_1(puzzle_input):
    _, marker_ix = find_marker_by_length(puzzle_input, marker_length=4)
    return marker_ix


def part_2(puzzle_input):
    _, marker_ix = find_marker_by_length(puzzle_input, marker_length=14)
    return marker_ix


if __name__ == "__main__":
    with input_fpath_for_day(6).open() as f:
        content = f.read()
        print("Day 6")
        print("Part 1:", part_1(content))
        print("Part 2:", part_2(content))
