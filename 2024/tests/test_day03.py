from aoc.days.day3 import DayRunner

SAMPLE_1 = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
SAMPLE_2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"


def test_sample_part1() -> None:
    assert DayRunner.part1(SAMPLE_1) == 161


def test_sample_part2() -> None:
    assert DayRunner.part2(SAMPLE_2) == 48
