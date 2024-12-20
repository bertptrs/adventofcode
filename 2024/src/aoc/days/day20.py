import itertools

import numpy

from . import SeparateRunner

DIRECTIONS = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
]

CHEATS = [
    (-2, 0),
    (2, 0),
    (0, -2),
    (0, 2),
]


def parse_path(input: str) -> dict[tuple[int, int], int]:
    grid = numpy.array(list(map(list, input.strip().split("\n"))))

    ys, xs = numpy.nonzero(grid == "S")
    sx, sy = int(xs[0]), int(ys[0])

    nx, ny = sx, sy

    path = {
        (sx, sy): 0,
    }

    while grid[ny, nx] != "E":
        x, y = nx, ny

        for dx, dy in DIRECTIONS:
            if grid[y + dy, x + dx] == "#" or (x + dx, y + dy) in path:
                continue
            nx = x + dx
            ny = y + dy
            break

        path[nx, ny] = len(path)
    return path


def get_savings(a: tuple[tuple[int, int], int], b: tuple[tuple[int, int], int]) -> int:
    (ax, ay), ad = a
    (bx, by), bd = b

    dist = abs(bx - ax) + abs(by - ay)
    if dist <= 20:
        return bd - ad - dist
    else:
        return 0


class DayRunner(SeparateRunner):
    @classmethod
    def part1(cls, input: str, limit: int = 100) -> int:
        path = parse_path(input)

        total = 0

        for (px, py), dist in path.items():
            for dx, dy in CHEATS:
                if (other := path.get((px + dx, py + dy))) is not None:
                    savings = dist - other - 2
                    if savings >= limit:
                        total += 1

        return total

    @classmethod
    def part2(cls, input: str, limit: int = 100) -> int:
        path = parse_path(input)

        return sum(
            get_savings(a, b) >= limit
            for a, b in itertools.combinations(path.items(), 2)
        )
