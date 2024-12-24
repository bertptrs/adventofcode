from aoc.days.day24 import DayRunner

from . import get_data


def test_sample_part1() -> None:
    assert DayRunner.part1(get_data(24)) == 2024
