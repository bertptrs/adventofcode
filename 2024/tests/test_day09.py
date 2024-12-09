from aoc.days.day9 import DayRunner

SAMPLE = "2333133121414131402"


def test_sample_part1() -> None:
    assert DayRunner.part1(SAMPLE) == 1928


def test_sample_part2() -> None:
    assert DayRunner.part2(SAMPLE) == 2858
