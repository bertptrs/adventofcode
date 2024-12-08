import os

from aoc.days.day6 import DayRunner


def get_data() -> str:
    sample = os.path.dirname(__file__) + "/samples/06.txt"
    with open(sample, mode="rt", encoding="utf-8") as f:
        return f.read()


def test_sample_part1() -> None:
    assert DayRunner.part1(get_data()) == 41


def test_sample_part2() -> None:
    assert DayRunner.part2(get_data()) == 6
