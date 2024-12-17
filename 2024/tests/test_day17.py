from aoc.days.day17 import DayRunner

from . import get_data


def test_sample_part1() -> None:
    assert DayRunner.part1(get_data(17, 1)) == "4,6,3,5,6,3,5,2,1,0"


def test_sample_part2() -> None:
    assert DayRunner.part2(get_data(17, 2)) == 117440
