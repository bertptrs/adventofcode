import numpy

from . import SeparateRunner


def does_loop(
    grid: numpy.array,
    x: int,
    y: int,
    dx: int,
    dy: int,
    visited: set[tuple[int, int, int, int]],
) -> bool:
    try:
        while True:
            while y + dy >= 0 and x + dx >= 0 and grid[y + dy, x + dx] == "#":
                dx, dy = -dy, dx

            x += dx
            y += dy

            if x < 0 or y < 0:
                return False

            pos = (x, y, dx, dy)

            if pos in visited:
                return True
            else:
                visited.add(pos)
    except IndexError:
        return False


class DayRunner(SeparateRunner):
    @classmethod
    def part1(cls, input: str) -> int:
        grid = input.strip().split("\n")

        for y, line in enumerate(grid):
            if (x := line.find("^")) != -1:
                break

        dx = 0
        dy = -1

        visited = {(x, y)}

        try:
            while True:
                nx = x + dx
                ny = y + dy

                if grid[ny][nx] == "#":
                    dx, dy = -dy, dx
                else:
                    x, y = nx, ny
                    visited.add((x, y))
        except IndexError:
            pass

        return len(visited)

    @classmethod
    def part2(cls, input: str) -> int:
        grid = numpy.array(list(map(list, input.strip().split("\n"))))
        y, x = numpy.where(grid == "^")

        y = y[0]
        x = x[0]

        dx = 0
        dy = -1
        loops = 0

        visited = {(x, y, dx, dy)}
        tiles_visited = {(x, y)}

        try:
            while True:
                while y + dy >= 0 and x + dx >= 0 and grid[y + dy, x + dx] == "#":
                    dx, dy = -dy, dx

                nx = x + dx
                ny = y + dy

                if nx < 0 or ny < 0:
                    break

                if (nx, ny) not in tiles_visited:
                    # check for a loop
                    grid[ny, nx] = "#"
                    if does_loop(grid, x, y, dx, dy, visited.copy()):
                        loops += 1
                        grid[ny, nx] = "L"
                    else:
                        grid[ny, nx] = "."

                x, y = nx, ny
                tiles_visited.add((x, y))
                visited.add((x, y, dx, dy))
        except IndexError:
            pass

        return loops
