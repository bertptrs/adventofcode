import pytest

from aoc.days.day4 import DayRunner

from . import get_data


@pytest.mark.parametrize(
    "input,answer",
    [
        (get_data(4, 1), 4),
        (get_data(4, 2), 18),
    ],
)
def test_sample_part1(input: str, answer: int) -> None:
    assert DayRunner.part1(input) == answer


def test_sample_part2() -> None:
    assert DayRunner.part2(get_data(4, 2)) == 9
