import math
from typing import List, TextIO

import numpy  # type: ignore


def read_input(data: TextIO) -> List[int]:
    line = next(data).strip()

    return [int(c) for c in line]


def simulate(numbers: List[int]) -> str:
    numbers = numpy.array(numbers)
    pattern = numpy.array([0, 1, 0, -1], dtype=numpy.int32)

    matrix = numpy.zeros((len(numbers), len(numbers)), dtype=numpy.int32)

    for i in range(len(numbers)):
        base = numpy.repeat(pattern, i + 1)
        needed_repetitions = math.ceil((len(numbers) + 1) / len(base))
        matrix[i, :] = numpy.tile(base, needed_repetitions)[1:len(numbers) + 1]

    for _ in range(100):
        numbers = numpy.abs(numpy.dot(matrix, numbers)) % 10

    return ''.join(str(s) for s in numbers[:8])


def simulate2(numbers: List[int]) -> str:
    numbers = numpy.tile(numpy.array(numbers, dtype=numpy.int32), 10000)
    starting_index = 0

    for n in numbers[:7]:
        starting_index *= 10
        starting_index += n

    assert starting_index > len(numbers) / 2

    numbers = numbers[starting_index:]

    for _ in range(100):
        numbers = numpy.abs(numpy.flip(numpy.cumsum(numpy.flip(numbers)))) % 10

    return ''.join(str(s) for s in numbers[:8])


def part1(data: TextIO) -> str:
    numbers = read_input(data)

    return simulate(numbers)


def part2(data: TextIO) -> str:
    numbers = read_input(data)

    return simulate2(numbers)
