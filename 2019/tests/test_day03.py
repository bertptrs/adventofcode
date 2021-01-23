from io import StringIO

import pytest

from aoc2019.day03 import part1, part2

SAMPLES = [
    "R8,U5,L5,D3\nU7,R6,D4,L4",
    "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
    "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
]


@pytest.mark.parametrize('paths,outcome', zip(SAMPLES, [6, 159, 135]))
def test_part1(paths: str, outcome: int):
    path_data = StringIO(paths)

    assert part1(path_data) == outcome


@pytest.mark.parametrize('paths,outcome', zip(SAMPLES, [30, 610, 410]))
def test_part2(paths: str, outcome: int):
    path_data = StringIO(paths)

    assert part2(path_data) == outcome
