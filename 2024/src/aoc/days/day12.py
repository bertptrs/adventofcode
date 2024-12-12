import numpy

from . import CombinedRunner

DIRECTIONS = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
]


class DayRunner(CombinedRunner):
    @classmethod
    def run_both(cls, input: str) -> tuple[int, int]:
        grid = numpy.array(list(map(list, input.strip().split("\n"))))

        score = 0
        score2 = 0

        for y in range(grid.shape[0]):
            for x in range(grid.shape[1]):
                if grid[y, x] == ".":
                    continue

                search = grid[y, x]
                grid[y, x] = "."

                todo = [(y, x)]
                cluster = {(y, x)}

                def enqueue(y, x):
                    if grid[y, x] == search:
                        grid[y, x] = "."
                        todo.append((y, x))
                        cluster.add((y, x))

                while todo:
                    cy, cx = todo.pop()

                    if cx > 0:
                        enqueue(cy, cx - 1)
                    if cy > 0:
                        enqueue(cy - 1, cx)

                    if cx < grid.shape[1] - 1:
                        enqueue(cy, cx + 1)
                    if cy < grid.shape[0] - 1:
                        enqueue(cy + 1, cx)

                side_length = sum(
                    sum((cy + dy, cx + dx) not in cluster for dy, dx in DIRECTIONS)
                    for cy, cx in cluster
                )

                corners = 0

                for cy, cx in cluster:
                    # Outer corners
                    corners += (cy, cx - 1) not in cluster and (
                        cy - 1,
                        cx,
                    ) not in cluster
                    corners += (cy, cx + 1) not in cluster and (
                        cy - 1,
                        cx,
                    ) not in cluster
                    corners += (cy, cx - 1) not in cluster and (
                        cy + 1,
                        cx,
                    ) not in cluster
                    corners += (cy, cx + 1) not in cluster and (
                        cy + 1,
                        cx,
                    ) not in cluster
                    # Inner corners
                    corners += (
                        (cy, cx - 1) in cluster
                        and (cy - 1, cx) in cluster
                        and (cy - 1, cx - 1) not in cluster
                    )
                    corners += (
                        (cy, cx + 1) in cluster
                        and (cy - 1, cx) in cluster
                        and (cy - 1, cx + 1) not in cluster
                    )
                    corners += (
                        (cy, cx - 1) in cluster
                        and (cy + 1, cx) in cluster
                        and (cy + 1, cx - 1) not in cluster
                    )
                    corners += (
                        (cy, cx + 1) in cluster
                        and (cy + 1, cx) in cluster
                        and (cy + 1, cx + 1) not in cluster
                    )

                score += side_length * len(cluster)
                score2 += corners * len(cluster)

        return (score, score2)
