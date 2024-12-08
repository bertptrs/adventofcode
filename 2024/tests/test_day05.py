from aoc.days.day5 import DayRunner

from . import get_data


def test_sample_part1() -> None:
    data = get_data(5)

    assert DayRunner.part1(data) == 143


def test_sample_part2() -> None:
    data = get_data(5)

    assert DayRunner.part2(data) == 123
