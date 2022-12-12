from aoc import input_fpath_for_day


def parse_input(puzzle_input):
    def movement_from(line):
        _line = line.split(" ")
        return (_line[0], int(_line[-1]))

    return list(map(movement_from, puzzle_input.splitlines()))


def move_knot(a, b):
    ax, ay = a
    bx, by = b
    dx = bx - ax
    dy = by - ay
    # If either distance > 1, move the knot
    if max(abs(dx), abs(dy)) > 1:
        dx = dx // abs(dx) if dx != 0 else 0
        dy = dy // abs(dy) if dy != 0 else 0
        return (ax + dx, ay + dy)

    return a


class Rope:
    def __init__(self, length=1):
        self.__length = length
        self.__knots = [(0, 0) for _ in range(length + 1)]

    @property
    def length(self):
        return self.__length

    def move(self, direction):
        (dx, dy) = {
            "R": (1, 0),
            "L": (-1, 0),
            "U": (0, -1),
            "D": (0, 1),
        }[direction]

        # Move the head
        x, y = self.__knots[0]
        self.__knots[0] = (x + dx, y + dy)
        # Move the rest
        for ix in range(1, self.length + 1):
            self.__knots[ix] = move_knot(self.__knots[ix], self.__knots[ix - 1])

    @property
    def tail(self):
        return self.__knots[-1]

    @property
    def head(self):
        return self.__knots[0]

    def knot(self, n):
        return self.__knots[n - 1]


def visualize_positions(positions: set):
    max_x = max([x for (x, _) in positions]) + 2
    min_x = min([x for (x, _) in positions]) - 1
    max_y = max([y for (_, y) in positions]) + 2
    min_y = min([y for (_, y) in positions]) - 1

    for y in range(min_y, max_y):
        for x in range(min_x, max_x):
            if (x, y) in positions:
                print("s" if (x, y) == (0, 0) else "#", end="")
            else:
                print(".", end="")
        print()


def part_1(puzzle_input):
    movements = parse_input(puzzle_input)
    submovements = [direction for (direction, nsteps) in movements for _ in range(nsteps)]
    rope = Rope()
    tail_positions = set()
    for direction in submovements:
        rope.move(direction)
        tail_positions.add(rope.tail)

    #  visualize_positions(tail_positions)
    return len(tail_positions)


def part_2(puzzle_input):
    movements = parse_input(puzzle_input)
    submovements = [direction for (direction, nsteps) in movements for _ in range(nsteps)]
    rope = Rope(length=9)
    tail_positions = set()
    for direction in submovements:
        rope.move(direction)
        tail_positions.add(rope.tail)

    #  visualize_positions(tail_positions)
    return len(tail_positions)


if __name__ == "__main__":
    with input_fpath_for_day(9).open() as f:
        content = """\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"""

        _content = """\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"""
        content = f.read()
        print("Day 9")
        print("Part 1:", part_1(content))
        print("Part 2:", part_2(content))
