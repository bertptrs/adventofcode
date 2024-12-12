import pytest

from aoc.days.day12 import DayRunner

from . import get_data


@pytest.mark.parametrize(
    "data,result",
    [(get_data(12, 1), 140), (get_data(12, 2), 772), (get_data(12, 3), 1930)],
)
def test_sample_part1(data: str, result: int) -> None:
    assert DayRunner.part1(data) == result


@pytest.mark.parametrize(
    "data,result",
    [(get_data(12, 1), 80), (get_data(12, 2), 436), (get_data(12, 3), 1206)],
)
def test_sample_part2(data: str, result: int) -> None:
    assert DayRunner.part2(data) == result
