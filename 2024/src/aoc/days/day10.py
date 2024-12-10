from collections import deque

import numpy

from . import SeparateRunner


class DayRunner(SeparateRunner):
    @classmethod
    def part1(cls, input: str) -> int:
        grid = numpy.array(
            [[int(v) for v in line] for line in input.strip().split("\n")]
        )

        width, height = grid.shape

        start_x, start_y = numpy.nonzero(grid == 0)

        todo = []

        reachable = 0

        for sx, sy in zip(start_x, start_y):
            todo.append((sx, sy))
            ways = numpy.zeros_like(grid, dtype=bool)

            def enqueue(x: int, y: int, val: int) -> None:
                if grid[x, y] == val + 1:
                    if not ways[x, y]:
                        todo.append((x, y))
                    ways[x, y] += True

            while todo:
                x, y = todo.pop()
                val = grid[x, y]
                if val == 9:
                    reachable += 1
                    continue

                if x > 0:
                    enqueue(x - 1, y, val)
                if y > 0:
                    enqueue(x, y - 1, val)
                if x < width - 1:
                    enqueue(x + 1, y, val)
                if y < height - 1:
                    enqueue(x, y + 1, val)

        return reachable

    @classmethod
    def part2(cls, input: str) -> int:
        grid = numpy.array(
            [[int(v) for v in line] for line in input.strip().split("\n")]
        )
        ways = numpy.zeros_like(grid)

        width, height = grid.shape

        start_x, start_y = numpy.nonzero(grid == 9)
        ways[grid == 9] = 1

        todo = deque((x, y) for x, y in zip(start_x, start_y))

        def enqueue(x: int, y: int, val: int, cur: int) -> None:
            if grid[x, y] == val - 1:
                if ways[x, y] == 0:
                    todo.append((x, y))
                ways[x, y] += cur

        while todo:
            x, y = todo.popleft()
            val = grid[x, y]
            cur = ways[x, y]

            if x > 0:
                enqueue(x - 1, y, val, cur)
            if y > 0:
                enqueue(x, y - 1, val, cur)
            if x < width - 1:
                enqueue(x + 1, y, val, cur)
            if y < height - 1:
                enqueue(x, y + 1, val, cur)

        return ways[grid == 0].sum()
