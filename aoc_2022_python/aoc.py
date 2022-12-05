import pathlib


def input_fpath_for_day(day_num: int) -> pathlib.Path:
    return pathlib.Path(__file__).parent.parent.resolve() / "inputs" / f"day{day_num}.txt"
