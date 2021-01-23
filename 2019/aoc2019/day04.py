import itertools
from typing import TextIO


def read_range(data: TextIO) -> range:
    a, b = next(data).split('-')

    return range(int(a), int(b) + 1)  # plus one because inclusive


def valid(number: int, strict: bool) -> bool:
    s = str(number)
    prev = '/'  # is smaller than '0'
    has_group = False

    if len(s) != 6:
        return False

    for k, g in itertools.groupby(s):
        if k < prev:
            return False

        prev = k

        amount = sum(1 for _ in g)

        if amount == 2 or not strict and amount > 2:
            has_group = True

    return has_group


def part1(data: TextIO) -> int:
    return sum(1 for password in read_range(data) if valid(password, False))


def part2(data: TextIO) -> int:
    return sum(1 for password in read_range(data) if valid(password, True))
