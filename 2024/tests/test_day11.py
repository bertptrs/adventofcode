from aoc.days.day11 import DayRunner

SAMPLE = "125 17"


def test_sample_part1() -> None:
    assert DayRunner.part1(SAMPLE) == 55312
