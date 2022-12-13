from aoc import input_fpath_for_day


def add_to_worry(rval):
    def _add_to(worry):
        return worry + rval

    return _add_to


def multiply_worry(rval):
    def _multiply_worry(worry):
        return worry * rval

    return _multiply_worry


def double_worry():
    def _double_worry(worry):
        return worry + worry

    return _double_worry


def square_worry():
    def _square_worry(worry):
        return worry * worry

    return _square_worry


class Monkey:
    def __init__(self, monkey_id, items, operation, test_val, monkey_true, monkey_false):
        self.id = monkey_id
        self.items = items
        self.operation = operation
        self.test_val = test_val
        self.monkey_true = monkey_true
        self.monkey_false = monkey_false
        self.inspection_count = 0

    def inspect(self, item):
        self.inspection_count += 1
        return self.operation(item)

    def check(self, worry):
        return self.monkey_true if worry % self.test_val == 0 else self.monkey_false


def monkey_from(block):
    lines = block.splitlines()
    monkey_id = int(lines[0].strip().removeprefix("Monkey ").strip(":"))
    items = list(map(int, lines[1].strip().removeprefix("Starting items: ").split(", ")))
    op, rval = lines[2].strip().removeprefix("Operation: new = old ").split(" ")
    if op == "*":
        operation = square_worry() if rval == "old" else multiply_worry(int(rval))
    elif op == "+":
        operation = double_worry() if rval == "old" else add_to_worry(int(rval))

    test_val = int(lines[3].strip().removeprefix("Test: divisible by "))
    monkey_true = int(lines[4].strip().split(" ")[-1])
    monkey_false = int(lines[5].strip().split(" ")[-1])

    return Monkey(
        monkey_id,
        items,
        operation,
        test_val,
        monkey_true,
        monkey_false,
    )


def parse_input(puzzle_input):
    return list(map(monkey_from, puzzle_input.strip().split("\n\n")))


def part_1(puzzle_input):
    monkeys = parse_input(puzzle_input)
    items_for_monkeys = dict()
    n_rounds = 20
    for _round in range(n_rounds):
        for monkey in monkeys:
            monkey.items.extend(items_for_monkeys.pop(monkey.id, list()))
            item = monkey.items.pop(0) if monkey.items else None
            while item is not None:
                worry_level = monkey.inspect(item)
                worry_level //= 3
                next_monkey_id = monkey.check(worry_level)
                thrown = items_for_monkeys.setdefault(next_monkey_id, list())
                thrown.append(worry_level)
                items_for_monkeys[next_monkey_id] = thrown
                item = monkey.items.pop(0) if monkey.items else None

    a, b = sorted([monkey.inspection_count for monkey in monkeys], reverse=True)[:2]
    return a * b


def part_2(puzzle_input):
    pass


if __name__ == "__main__":
    with input_fpath_for_day(11).open() as f:
        content = """\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"""

        content = f.read()
        print("Day 11")
        print("Part 1:", part_1(content))
        print("Part 2:", part_2(content))
