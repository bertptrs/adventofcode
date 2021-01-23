from collections import Counter
from typing import Iterable, TextIO

import numpy  # type: ignore


def parse_layers(width: int, height: int, data: TextIO) -> Iterable[numpy.array]:
    chunk_size = width * height

    content = next(data).strip()

    for pos in range(0, len(content), chunk_size):
        yield numpy.array([int(c) for c in content[pos:pos + chunk_size]])


def part1(data: TextIO) -> int:
    best_layer: Counter[int] = min((Counter(layer) for layer in parse_layers(25, 6, data)), key=lambda c: c[0])

    return best_layer[1] * best_layer[2]


def format_row(row: Iterable[int]) -> str:
    return ''.join('#' if p == 1 else ' ' for p in row)


def part2(data: TextIO) -> str:
    layers = list(parse_layers(25, 6, data))
    background = numpy.zeros(25 * 6, numpy.int8)

    for layer in reversed(layers):
        background[layer != 2] = layer[layer != 2]

    return '\n'.join(format_row(row) for row in background.reshape(6, 25))
