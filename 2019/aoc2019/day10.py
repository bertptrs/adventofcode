import math
from collections import defaultdict
from typing import TextIO, Tuple

import numpy  # type: ignore


def read_asteroids(data: TextIO) -> Tuple[numpy.array, numpy.array]:
    xs = []
    ys = []
    for y, line in enumerate(data):
        for x, c in enumerate(line):
            if c == '#':
                xs.append(x)
                ys.append(y)

    return numpy.array(xs), -numpy.array(ys)


def asteroids_visible(x: int, y: int, xs: numpy.array, ys: numpy.array) -> int:
    dx = xs - x
    dy = ys - y

    div = numpy.abs(numpy.gcd(dx, dy))

    mask = div != 0

    dx[mask] //= div[mask]
    dy[mask] //= div[mask]

    unique = set(zip(dx, dy))

    return len(unique) - 1  # need to ignore the point itself


def part1(data: TextIO) -> int:
    xs, ys = read_asteroids(data)

    return max(asteroids_visible(x, y, xs, ys) for x, y in zip(xs, ys))


def part2(data: TextIO) -> int:
    xs, ys = read_asteroids(data)

    cx, cy = max(zip(xs, ys), key=lambda c: asteroids_visible(c[0], c[1], xs, ys))

    dx = xs - cx
    dy = ys - cy

    angles = numpy.arctan2(dy, dx)
    distances = numpy.abs(numpy.copy(dx)) + numpy.abs(numpy.copy(dy))

    to_shoot = defaultdict(list)

    for angle, distance, dx, dy in zip(angles, distances, dx, dy):
        if distance == 0:
            # The point itself
            continue

        to_shoot[angle].append((distance, dx, dy))

    for distances in to_shoot.values():
        distances.sort(reverse=True)

    unique_angles = sorted(set(angles), reverse=True)

    shot = 0

    # First shoot from up clockwise
    for angle in unique_angles:
        if angle > math.pi / 2:
            continue

        shot += 1

        _, dx, dy = to_shoot[angle].pop()

    # Repeatedly shoot until you reach #200
    while True:
        for angle in unique_angles:
            if not to_shoot[angle]:
                # Nothing left to shoot
                continue

            shot += 1

            _, dx, dy = to_shoot[angle].pop()

            if shot == 200:
                x = cx + dx
                y = -(cy + dy)
                return 100 * x + y

