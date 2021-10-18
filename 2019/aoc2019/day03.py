import itertools
from typing import Dict, TextIO


def compute_points(line: str) -> Dict[complex, int]:
    points: Dict[complex, int] = {}
    steps = itertools.count(1)

    pos = complex(0)

    directions = {
        'U': 1j,
        'R': 1,
        'D': -1j,
        'L': -1,
    }

    for move in line.strip().split(','):
        direction = directions[move[0]]

        for _ in range(int(move[1:])):
            pos += direction

            points.setdefault(pos, next(steps))

    return points


def part1(data: TextIO) -> int:
    points_a = compute_points(next(data))
    points_b = compute_points(next(data))

    in_common = set(points_a.keys()) & set(points_b.keys())

    return int(min(abs(c.imag) + abs(c.real) for c in in_common))


def part2(data: TextIO) -> int:
    points_a = compute_points(next(data))
    points_b = compute_points(next(data))

    in_common = set(points_a.keys()) & set(points_b.keys())

    return min(points_a[pos] + points_b[pos] for pos in in_common)
