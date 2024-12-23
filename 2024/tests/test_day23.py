from aoc.days.day23 import DayRunner

from . import get_data


def test_sample_part1() -> None:
    assert DayRunner.part1(get_data(23)) == 7


def test_sample_part2() -> None:
    assert DayRunner.part2(get_data(23)) == "co,de,ka,ta"
