import heapq

import numpy

from . import SeparateRunner

TURNS = (
    (-1, 1),
    (1, -1),
)


class DayRunner(SeparateRunner):
    @classmethod
    def part1(cls, input: str) -> int:
        grid = numpy.array([list(line) for line in input.strip().split("\n")])

        y, x = numpy.where(grid == "S")
        x, y = x[0], y[0]

        todo = [(0, x, y, 1, 0)]
        best = {
            (x, y, 1, 0): 0,
        }

        def enqueue(dist, x, y, dx, dy):
            if grid[y, x] == "#":
                return

            if (x, y, dx, dy) not in best or best[x, y, dx, dy] > dist:
                best[x, y, dx, dy] = dist
                heapq.heappush(todo, (dist, x, y, dx, dy))

        while todo:
            dist, x, y, dx, dy = heapq.heappop(todo)

            if best[x, y, dx, dy] < dist:
                continue

            if grid[y, x] == "E":
                return dist

            enqueue(dist + 1, x + dx, y + dy, dx, dy)
            enqueue(dist + 2001, x - dx, y - dy, dx, dy)

            for tx, ty in TURNS:
                ndx = dy * ty
                ndy = dx * ty

                enqueue(dist + 1001, x + ndx, y + ndy, ndx, ndy)

        raise ValueError("Did not find path to exit")

    @classmethod
    def part2(cls, input: str) -> int:
        pass
