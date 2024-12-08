from aoc.days.day8 import DayRunner

from . import get_data


def test_sample_part1() -> None:
    assert DayRunner.part1(get_data(8)) == 14


def test_sample_part2() -> None:
    assert DayRunner.part2(get_data(8)) == 34
