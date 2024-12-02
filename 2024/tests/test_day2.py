import os

from aoc.days.day2 import DayRunner, is_savable


def get_data() -> str:
    sample = os.path.dirname(__file__) + "/samples/02.txt"
    with open(sample, mode="rt", encoding="utf-8") as f:
        return f.read()


def test_individual_samples() -> None:
    assert is_savable("7 6 4 2 1")
    assert not is_savable("1 2 7 8 9")
    assert not is_savable("9 7 6 2 1")
    assert is_savable("1 3 2 4 5")
    assert is_savable("8 6 4 4 1")
    assert is_savable("1 3 6 7 9")


def test_sample_part1() -> None:
    assert DayRunner.part1(get_data()) == 2


def test_sample_part2() -> None:
    assert DayRunner.part2(get_data()) == 4
