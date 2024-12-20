from aoc.days.day20 import DayRunner

from . import get_data


def test_sample_part1() -> None:
    assert DayRunner.part1(get_data(20), limit=1) == 44


def test_sample_part2() -> None:
    assert DayRunner.part2(get_data(20), limit=50) == 285
