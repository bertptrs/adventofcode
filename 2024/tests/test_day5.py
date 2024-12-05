import os

from aoc.days.day5 import DayRunner


def get_data() -> str:
    sample = os.path.dirname(__file__) + "/samples/05.txt"
    with open(sample, mode="rt", encoding="utf-8") as f:
        return f.read()


def test_sample_part1() -> None:
    data = get_data()

    assert DayRunner.part1(data) == 143


def test_sample_part2() -> None:
    data = get_data()

    assert DayRunner.part2(data) == 123
