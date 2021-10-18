import pytest

from aoc2019.day16 import simulate, simulate2


@pytest.mark.parametrize('data,correct', [
    ('80871224585914546619083218645595', '24176176'),
    ('19617804207202209144916044189917', '73745418'),
    ('69317163492948606335995924319873', '52432133'),
])
def test_sample_part1(data: str, correct: str):
    numbers = [int(c) for c in data]

    assert simulate(numbers) == correct


@pytest.mark.parametrize('data,correct', [
    ('03036732577212944063491565474664', '84462026'),
    ('02935109699940807407585447034323', '78725270'),
    ('03081770884921959731165446850517', '53553731'),
])
def test_sample_part2(data: str, correct: str):
    numbers = [int(c) for c in data]

    assert simulate2(numbers) == correct
