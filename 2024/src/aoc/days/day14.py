import itertools
import math
import re

import numpy

from . import SeparateRunner

NASTY_REGEX = re.compile(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)")


class DayRunner(SeparateRunner):
    @classmethod
    def part1(cls, input: str, width: int = 101, height: int = 103) -> int:
        points = NASTY_REGEX.findall(input)

        quadrants = [0] * 4

        x_middle = width // 2
        y_middle = height // 2

        for point in points:
            x, y, dx, dy = map(int, point)

            rx = ((x + dx * 100) % width + width) % width
            ry = ((y + dy * 100) % height + height) % height

            match rx:
                case _ if rx < x_middle:
                    xq = 0
                case _ if rx > x_middle:
                    xq = 1
                case _:
                    continue

            match ry:
                case _ if ry < y_middle:
                    yq = 0
                case _ if ry > y_middle:
                    yq = 1
                case _:
                    continue

            quadrants[2 * yq + xq] += 1

        return math.prod(quadrants)

    @classmethod
    def part2(cls, input: str) -> int:
        width = 101
        height = 103

        points = NASTY_REGEX.findall(input)
        points_fast = numpy.array([list(map(int, point)) for point in points])

        positions = points_fast[:, 0:2]
        velocities = points_fast[:, 2:]

        target = len(velocities)

        # Assumption: when the easter egg happens, no robots overlap, and this is the
        # only time this happens. There is no reason this should work but it does.
        mod_base = numpy.array([width, height])
        for i in itertools.count(1):
            positions += velocities

            positions %= mod_base
            positions += mod_base
            positions %= mod_base

            if len(numpy.unique(positions, axis=0)) == target:
                # TODO: print the Christmas tree, Eric prepared it for us so nicely
                return i
