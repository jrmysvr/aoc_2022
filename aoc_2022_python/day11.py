from aoc import input_fpath_for_day
from queue import Queue, Empty
from collections import OrderedDict
from multiprocessing import Pool


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
        self._items = Queue()

        for item in items:
            self._items.put(item)

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

    def catch(self, item):
        self._items.put(item)

    def get_next_item(self):
        try:
            return self._items.get(block=False)
        except Empty:
            return None

    @property
    def items(self):
        _items = list()
        item = self.get_next_item()
        while item is not None:
            _items.append(item)
            item = self.get_next_item()

        return _items


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
    _monkeys = {monkey.id: monkey for monkey in monkeys}
    n_rounds = 20
    for _round in range(n_rounds):
        for _monkey in monkeys:
            monkey = _monkeys[_monkey.id]
            item = monkey.get_next_item()
            while item is not None:
                worry_level = monkey.inspect(item)
                worry_level //= 3
                next_monkey_id = monkey.check(worry_level)
                _monkeys[next_monkey_id].catch(worry_level)
                item = monkey.get_next_item()

    print([monkey.inspection_count for monkey in monkeys])
    a, b = sorted([monkey.inspection_count for monkey in monkeys], reverse=True)[:2]
    return a * b


def brute_force_part_2(puzzle_input):
    monkeys = parse_input(puzzle_input)
    all_items = list()
    for monkey in monkeys:
        for item in monkey.items:
            all_items.append(item)
            ix = len(all_items) - 1
            monkey.catch(ix)

    _monkeys = {monkey.id: monkey for monkey in monkeys}

    n_rounds = 10000
    for _round in range(n_rounds):
        for _monkey in monkeys:
            monkey = _monkeys[_monkey.id]
            item_ix = monkey.get_next_item()
            while item_ix is not None:
                item = all_items[item_ix]
                worry_level = monkey.inspect(item)
                all_items[item_ix] = worry_level
                next_monkey_id = monkey.check(worry_level)
                _monkeys[next_monkey_id].catch(item_ix)
                item_ix = monkey.get_next_item()

    a, b = sorted([monkey.inspection_count for monkey in monkeys], reverse=True)[:2]
    return a * b


def calc(item_and_ix):
    _item_ix, item = item_and_ix
    with input_fpath_for_day(11).open() as f:
        puzzle_input = f.read()
    monkeys = parse_input(puzzle_input)
    all_items = list()
    for monkey in monkeys:
        for item in monkey.items:
            all_items.append(item)
            ix = len(all_items) - 1
            if ix == _item_ix:
                monkey.catch(ix)

    _monkeys = OrderedDict([(monkey.id, monkey) for monkey in monkeys])

    n_rounds = 10000
    for _round in range(n_rounds):
        for _monkey in monkeys:
            monkey = _monkeys[_monkey.id]
            item_ix = monkey.get_next_item()
            while item_ix is not None:
                item = all_items[item_ix]
                worry_level = monkey.inspect(item)
                all_items[item_ix] = worry_level
                next_monkey_id = monkey.check(worry_level)
                _monkeys[next_monkey_id].catch(item_ix)
                item_ix = monkey.get_next_item()

    return [monkey.inspection_count for monkey in _monkeys.values()]


def part_2(puzzle_input):
    all_items = list()
    monkeys = parse_input(puzzle_input)
    for monkey in monkeys:
        for item in monkey.items:
            all_items.append(item)
            ix = len(all_items) - 1
            monkey.catch(ix)

    all_items = enumerate(all_items)
    with Pool(10) as p:
        counts = [0] * len(monkeys)
        for result in p.map(calc, all_items):
            counts = [c + r for (c, r) in zip(counts, result)]

    a, b = sorted(counts, reverse=True)[:2]
    return a * b


def test_content():
    return """\
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


if __name__ == "__main__":
    with input_fpath_for_day(11).open() as f:
        content = f.read()
        print("Day 11")
        print("Part 1:", part_1(content))
        print("Part 2:", part_2(content))
