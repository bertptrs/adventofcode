import os

from aoc.days.day1 import DayRunner


def get_data() -> str:
    sample = os.path.dirname(__file__) + "/samples/01.txt"
    with open(sample, mode="rt", encoding="utf-8") as f:
        return f.read()


def test_sample_part1() -> None:
    assert DayRunner.part1(get_data()) == 11


def test_sample_part2() -> None:
    assert DayRunner.part2(get_data()) == 31
