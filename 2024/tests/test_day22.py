from aoc.days.day22 import DayRunner

from . import get_data


def test_sample_part1() -> None:
    assert DayRunner.part1(get_data(22, 1)) == 37327623


def test_sample_part2() -> None:
    assert DayRunner.part2(get_data(22, 2)) == 23
