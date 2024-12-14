from aoc.days.day14 import DayRunner

from . import get_data


def test_sample_part1() -> None:
    assert DayRunner.part1(get_data(14), width=11, height=7) == 12
