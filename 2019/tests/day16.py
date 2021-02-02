import pytest

from aoc2019.day16 import simulate


@pytest.mark.parametrize('data,correct', [
    ('80871224585914546619083218645595', '24176176'),
    ('19617804207202209144916044189917', '73745418'),
    ('69317163492948606335995924319873', '52432133'),
])
def test_sample_part1(data: str, correct: str):
    numbers = [int(c) for c in data]

    assert simulate(numbers) == correct
    pass
