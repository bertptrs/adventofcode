import pytest

from aoc.days.day16 import DayRunner

from . import get_data


@pytest.mark.parametrize(
    "data,result",
    [
        (get_data(16, 1), 7036),
        (get_data(16, 2), 11048),
    ],
)
def test_sample_part1(data: str, result: int) -> None:
    assert DayRunner.part1(data) == result


@pytest.mark.parametrize(
    "data,result",
    [
        (get_data(16, 1), 45),
        (get_data(16, 2), 64),
    ],
)
def test_sample_part2(data: str, result: int) -> None:
    assert DayRunner.part2(data) == result
