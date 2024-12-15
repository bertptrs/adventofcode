import pytest

from aoc.days.day15 import DayRunner

from . import get_data


@pytest.mark.parametrize(
    "data,result",
    [
        (get_data(15, 1), 10092),
        (get_data(15, 2), 2028),
    ],
)
def test_sample_part1(data: str, result: int) -> None:
    assert DayRunner.part1(data) == result


@pytest.mark.parametrize(
    "data,result",
    [
        (get_data(15, 1), 9021),
        (get_data(15, 3), 618),
    ],
)
def test_sample_part2(data: str, result: int) -> None:
    assert DayRunner.part2(data) == result
