import pytest
import numpy  # type: ignore

from aoc2019.day12 import simulate_moons, find_repetition

SAMPLES = [
    numpy.array([[-1, 0, 2], [2, -10, -7], [4, -8, 8], [3, 5, -1]]),
    numpy.array([[-8, -10, 0], [5, 5, 10], [2, -7, 3], [9, -8, -3]]),
]


@pytest.mark.parametrize('iterations,result,moons', [
    (10, 179, numpy.copy(SAMPLES[0])),
    (100, 1940, numpy.copy(SAMPLES[1])),
])
def test_kinetic_energy(moons: numpy.array, iterations: int, result: int) -> None:
    assert simulate_moons(moons, iterations) == result


@pytest.mark.parametrize('outcome,moons', [
    (2772, numpy.copy(SAMPLES[0])),
    (4686774924, numpy.copy(SAMPLES[1])),
])
def test_repetition(moons: numpy.array, outcome: int) -> None:
    assert find_repetition(moons) == outcome
