import numpy

from . import SeparateRunner


class DayRunner(SeparateRunner):
    @classmethod
    def part1(cls, input: str) -> int:
        grid = numpy.array(list(map(list, input.strip().split("\n"))))

        found = 0

        directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]

        word = "XMAS"

        for y in range(grid.shape[0]):
            for x in range(grid.shape[1]):
                if grid[y, x] != "X":
                    continue

                for dx, dy in directions:
                    end_x = x + 3 * dx
                    end_y = y + 3 * dy

                    if (
                        end_x < 0
                        or end_x >= grid.shape[1]
                        or end_y < 0
                        or end_y >= grid.shape[0]
                    ):
                        continue

                    if all(
                        grid[y + i * dy, x + i * dx] == c for i, c in enumerate(word)
                    ):
                        found += 1

        return found

    @classmethod
    def part2(cls, input: str) -> int:
        grid = numpy.array(list(map(list, input.strip().split("\n"))))

        found = 0

        magic = ord("M") ^ ord("S")

        for y in range(1, grid.shape[0] - 1):
            for x in range(1, grid.shape[1] - 1):
                if grid[y, x] != "A":
                    continue

                first_diag = ord(grid[y - 1, x - 1]) ^ ord(grid[y + 1, x + 1])
                secnd_diag = ord(grid[y - 1, x + 1]) ^ ord(grid[y + 1, x - 1])

                if first_diag == magic and secnd_diag == magic:
                    found += 1

        return found
