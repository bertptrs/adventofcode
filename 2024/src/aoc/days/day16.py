import heapq

import numpy

from . import CombinedRunner

TURNS = (
    (-1, 1),
    (1, -1),
)


class DayRunner(CombinedRunner):
    @classmethod
    def run_both(cls, input: str) -> tuple[int, int]:
        grid = numpy.array([list(line) for line in input.strip().split("\n")])

        y, x = numpy.where(grid == "S")
        x, y = x[0], y[0]

        todo = [(0, x, y, 1, 0)]
        best = {
            (x, y, 1, 0): (0, []),
        }

        def enqueue(dist, x, y, dx, dy, cx, cy, cdx, cdy):
            if grid[y, x] == "#":
                return

            if (x, y, dx, dy) not in best or best[x, y, dx, dy][0] > dist:
                best[x, y, dx, dy] = (dist, [(cx, cy, cdx, cdy)])
                heapq.heappush(todo, (dist, x, y, dx, dy))
            elif best[x, y, dx, dy][0] == dist:
                best[x, y, dx, dy][1].append((cx, cy, cdx, cdy))

        shortest_dist = None
        finishes = set()

        while todo:
            dist, x, y, dx, dy = heapq.heappop(todo)

            if best[x, y, dx, dy][0] < dist:
                continue

            if shortest_dist is not None and shortest_dist < dist:
                break

            if grid[y, x] == "E":
                shortest_dist = dist
                finishes.add((x, y, dx, dy))

            enqueue(dist + 1, x + dx, y + dy, dx, dy, x, y, dx, dy)
            enqueue(dist + 2001, x - dx, y - dy, dx, dy, x, y, dx, dy)

            for tx, ty in TURNS:
                ndx = dy * ty
                ndy = dx * ty

                enqueue(dist + 1001, x + ndx, y + ndy, ndx, ndy, x, y, dx, dy)

        assert shortest_dist is not None, "Should find a path to the exit"

        visited_tiles = {(x, y) for x, y, _, _ in finishes}
        todo2 = [f for f in finishes]
        visited_states = set(todo2)

        while todo2:
            state = todo2.pop()

            for prev in best[state][1]:
                if prev not in visited_states:
                    visited_states.add(prev)
                    visited_tiles.add((prev[0], prev[1]))
                    todo2.append(prev)

        return shortest_dist, len(visited_tiles)
