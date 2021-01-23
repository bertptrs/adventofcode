from aoc2019.day01 import fuel_required, recursive_fuel_required

import pytest


@pytest.mark.parametrize('weight,required', [
    (12, 2),
    (14, 2),
    (1969, 654),
    (100756, 33583)
])
def test_fuel_required(weight: int, required: int) -> None:
    assert fuel_required(weight) == required


@pytest.mark.parametrize('weight,required', [
    (14, 2),
    (1969, 966),
    (100756, 50346)
])
def test_fuel_required_recursive(weight: int, required: int) -> None:
    assert recursive_fuel_required(weight) == required
