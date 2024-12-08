import itertools
import math
from collections import defaultdict

import numpy

from . import CombinedRunner


def simplify(vec: numpy.array) -> numpy.array:
    if vec[0] == 0:
        return numpy.array([0, 1])
    elif vec[1] == 0:
        return numpy.array([0, 1])
    else:
        div = math.gcd(*vec)
        return vec // div


class DayRunner(CombinedRunner):
    @classmethod
    def run_both(cls, input: str) -> int:
        grid = input.strip().split("\n")
        height = len(grid)
        width = len(grid[0])

        antennae = defaultdict(list)

        for y, line in enumerate(grid):
            for x, c in enumerate(line):
                if c != ".":
                    antennae[c].append(numpy.array([x, y]))

        antinodes = set()
        antinodes2 = set()

        def in_bounds(node: numpy.array) -> bool:
            return 0 <= node[0] < width and 0 <= node[1] < height

        def add(node: numpy.array):
            if in_bounds(node):
                antinodes.add(tuple(node))

        def walk(start: numpy.array, step: numpy.array):
            for pos in itertools.count(start, step):
                if in_bounds(pos):
                    antinodes2.add(tuple(pos))
                else:
                    break

        for values in antennae.values():
            for a, b in itertools.combinations(values, 2):
                add(2 * a - b)
                add(2 * b - a)

                step = simplify(b - a)
                walk(b, step)
                walk(a, -step)

        return len(antinodes), len(antinodes2)
