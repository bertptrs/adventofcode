import os

import pytest

from aoc.days.day4 import DayRunner


def get_data(which: int) -> str:
    sample = os.path.dirname(__file__) + f"/samples/04.{which}.txt"
    with open(sample, mode="rt", encoding="utf-8") as f:
        return f.read()


@pytest.mark.parametrize(
    "input,answer",
    [
        (get_data(1), 4),
        (get_data(2), 18),
    ],
)
def test_sample_part1(input: str, answer: int) -> None:
    assert DayRunner.part1(input) == answer


def test_sample_part2() -> None:
    assert DayRunner.part2(get_data(2)) == 9
