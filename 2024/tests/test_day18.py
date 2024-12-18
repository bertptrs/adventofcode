from aoc.days.day18 import DayRunner

from . import get_data


def test_sample_part1() -> None:
    assert DayRunner.part1(get_data(18), width=7, height=7, limit=12) == 22


def test_sample_part2() -> None:
    assert DayRunner.part2(get_data(18), width=7, height=7) == "6,1"
