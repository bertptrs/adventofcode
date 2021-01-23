import pytest

from aoc2019.day04 import valid


@pytest.mark.parametrize('number,strict,expected', [
    (122345, False, True),
    (111123, False, True),
    (111111, False, True),
    (223450, False, False),
    (123789, False, False),
    (112233, True, True),
    (123444, True, False),
    (111122, True, True)
])
def test_valid(number: int, strict: bool, expected: bool) -> None:
    assert valid(number, strict) == expected
