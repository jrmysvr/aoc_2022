from aoc import input_fpath_for_day


def get_cells_near(pos, grid, ignore_pos=None):
    movements = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    r, c = pos
    cells = list()
    for (dr, dc) in movements:
        if dr == 0 and dc == 0:
            continue
        _r = r + dr
        _c = c + dc
        if (0 <= _r < len(grid)) and (0 <= _c < len(grid[0])):
            if (_r, _c) != ignore_pos:
                cells.append((_r, _c))

    return cells


def direction_to_cell(_from, _to):
    (ar, ac) = _from
    (br, bc) = _to
    return {
        (-1, 0): "^",
        (1, 0): "v",
        (0, -1): "<",
        (0, 1): ">",
    }[(br - ar, bc - ac)]


def paths_from_puzzle_input(puzzle_input):
    start = (0, 0)
    end = (0, 0)
    START = "S"
    START_VAL = 0
    END = "E"
    END_VAL = 27
    grid = list(map(list, puzzle_input.strip().splitlines()))
    for rx, row in enumerate(grid):
        for cx, cell in enumerate(row):
            if cell == START:
                start = (rx, cx)
                grid[rx][cx] = START_VAL
            elif cell == END:
                end = (rx, cx)
                grid[rx][cx] = END_VAL
            else:
                cell = grid[rx][cx]
                grid[rx][cx] = ord(cell) - ord("a") + 1

    movements = [["." for _ in range(len(grid[0]))] for _ in range(len(grid))]
    pos = start
    cells = get_cells_near(pos, grid)
    current = grid[pos[0]][pos[1]]
    paths = [[pos]]
    paths = [path + [cell] for path in paths for cell in cells]
    visited = set()
    some_visited = True
    end_paths = list()
    while some_visited:
        some_visited = False
        new_paths = list()
        for _ in range(len(paths)):
            path = paths.pop()
            pos = path[-1]
            current = grid[pos[0]][pos[1]]
            if current == END_VAL:
                end_paths.append(path)
                continue
            cells = get_cells_near(pos, grid, ignore_pos=path[-2])
            for cx, cell in enumerate(cells):
                destination = grid[cell[0]][cell[1]]
                if destination - current <= 1 and cell not in visited:
                    some_visited = True
                    _path = list(path) + [cell]
                    visited.add(cell)
                    new_paths.append(_path)

        paths = list(new_paths)

    end_paths = sorted([path for path in end_paths if path[-1] == end], key=lambda p: len(p))

    # Display the shorted path
    shortest = sorted(end_paths, key=lambda p: len(p))[0]
    movements = [["." for _ in range(len(grid[0]))] for _ in range(len(grid))]
    prev = shortest[0]
    _end = shortest[-1]
    movements[prev[0]][prev[1]] = "S"
    for curr in shortest[1:-1]:
        movements[curr[0]][curr[1]] = direction_to_cell(prev, curr)
        prev = curr
    movements[_end[0]][_end[1]] = "E"

    print("\n".join("".join(row) for row in movements))

    return end_paths


def part_1(puzzle_input):
    # Calculate the shortest path (steps between positions, hence the minus 1)
    return min(map(len, paths_from_puzzle_input(puzzle_input))) - 1


def part_2(puzzle_input):
    pass


if __name__ == "__main__":
    with input_fpath_for_day(12).open() as f:
        content = """\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"""
        content = f.read()
        print("Day 12")
        print("Part 1:", part_1(content))
        print("Part 2:", part_2(content))
