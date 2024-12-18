from collections import deque

from . import SeparateRunner


def parse_input(data: str) -> list[tuple[int, int]]:
    return [tuple(map(int, line.split(","))) for line in data.strip().split("\n")]


def find_exit(fallen: set[tuple[int, int]], width: int, height: int) -> int | None:
    todo = deque([(0, 0, 0)])

    best = {(0, 0): 0}

    def enqueue(dist: int, x: int, y: int):
        # print(f"trying {x},{y}")
        if (x, y) in fallen:
            return

        if (x, y) not in best or best[x, y] > dist:
            best[x, y] = dist
            todo.append((dist, x, y))

    while todo:
        dist, x, y = todo.popleft()
        # print(x, y)

        if x == width - 1 and y == height - 1:
            return dist

        if x > 0:
            enqueue(dist + 1, x - 1, y)

        if x + 1 < width:
            enqueue(dist + 1, x + 1, y)

        if y > 0:
            enqueue(dist + 1, x, y - 1)

        if y + 1 < height:
            enqueue(dist + 1, x, y + 1)


class DayRunner(SeparateRunner):
    @classmethod
    def part1(
        cls, input: str, width: int = 71, height: int = 71, limit: int = 1024
    ) -> int:
        falling = parse_input(input)

        return find_exit(set(falling[:limit]), width, height)

    @classmethod
    def part2(cls, input: str, width: int = 71, height: int = 71) -> str:
        falling = parse_input(input)

        lower = 0
        upper = len(falling)

        while lower < upper:
            mid = lower + (upper - lower) // 2

            if find_exit(set(falling[:mid]), width, height) is not None:
                lower = mid + 1
            else:
                upper = mid

        first_blocker = falling[lower - 1]

        return f"{first_blocker[0]},{first_blocker[1]}"
