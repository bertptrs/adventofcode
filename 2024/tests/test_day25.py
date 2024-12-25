from aoc.days.day25 import DayRunner

from . import get_data


def test_sample_part1() -> None:
    assert DayRunner.part1(get_data(25)) == 3
