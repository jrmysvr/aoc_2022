import pathlib


def calc_shape_score_from(me):
    return {
        "X": 1,
        "Y": 2,
        "Z": 3,
    }[me]


def calc_play_score_from(opponent, me):
    loss = 0
    draw = 3
    win = 6

    return {
        "A|X": draw,
        "B|X": loss,
        "C|X": win,
        "A|Y": win,
        "B|Y": draw,
        "C|Y": loss,
        "A|Z": loss,
        "B|Z": win,
        "C|Z": draw,
    }[f"{opponent}|{me}"]


def calc_play_from(opponent, ending):
    return {
        "A|X": "Z",
        "B|X": "X",
        "C|X": "Y",
        "A|Y": "X",
        "B|Y": "Y",
        "C|Y": "Z",
        "A|Z": "Y",
        "B|Z": "Z",
        "C|Z": "X",
    }[f"{opponent}|{ending}"]


def score_round(round_tup):
    op, me = round_tup
    shape_score = calc_shape_score_from(me)
    play_score = calc_play_score_from(op, me)

    return shape_score + play_score


def score_round_part_2(round_tup):
    op, ending = round_tup
    me = calc_play_from(op, ending)
    shape_score = calc_shape_score_from(me)
    play_score = calc_play_score_from(op, me)

    return shape_score + play_score


if __name__ == "__main__":
    input_fpath = pathlib.Path(__file__).parent / ".." / "inputs" / "day2.txt"
    with input_fpath.open() as f:
        content = f.read().splitlines()
        print("Day 2")
        print("Part 1:", sum(map(score_round, map(str.split, content))))
        print("Part 2:", sum(map(score_round_part_2, map(str.split, content))))
