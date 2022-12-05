from aoc import input_fpath_for_day


def get_shared_item(rucksack):
    size = len(rucksack)
    fst, snd = rucksack[: size // 2], rucksack[size // 2 :]
    return set(fst).intersection(set(snd)).pop()


def get_shared_item_part_2(grouped_rucksack):
    items = set(grouped_rucksack[0])
    for i in range(1, len(grouped_rucksack)):
        items = items.intersection(set(grouped_rucksack[i]))

    return items.pop()


def prioritize(item):
    """
    >>> prioritize('a')
    1
    >>> prioritize('A')
    27
    """
    if item.isupper():
        return ord(item) - ord("A") + 27
    return ord(item) - ord("a") + 1


def windowed(stuff, window_size):
    w = [stuff[i : i + window_size] for i in range(0, len(stuff) - window_size + 1, window_size)]
    return w


def part_1(content):
    return sum(map(prioritize, map(get_shared_item, content)))


def part_2(content):
    return sum(map(prioritize, map(get_shared_item_part_2, windowed(content, 3))))


if __name__ == "__main__":
    with input_fpath_for_day(3).open() as f:
        content = """vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw""".strip().splitlines()
        content = f.read().strip().splitlines()
        print("Day 3")
        print("Part 1:", part_1(content))
        print("Part 2:", part_2(content))
