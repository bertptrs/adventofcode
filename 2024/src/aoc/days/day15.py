import itertools

import numpy

from . import SeparateRunner


def parse_input(data: str) -> tuple[numpy.array, str]:
    grid, steps = data.split("\n\n")

    grid_split = numpy.array([list(line) for line in grid.split("\n")])

    steps = "".join(steps.split("\n"))

    return grid_split, steps


class DayRunner(SeparateRunner):
    @classmethod
    def part1(cls, input: str) -> None:
        grid, steps = parse_input(input)

        y, x = numpy.where(grid == "@")
        x, y = x[0], y[0]

        for c in steps:
            match c:
                case "^":
                    dx, dy = 0, -1
                case ">":
                    dx, dy = 1, 0
                case "<":
                    dx, dy = -1, 0
                case "v":
                    dx, dy = 0, 1
                case other:
                    raise ValueError(f"Invalid movement: {other}")

            match grid[y + dy, x + dx]:
                case "#":
                    continue
                case "O":
                    crashed = False
                    for dist in itertools.count(2):
                        match grid[y + dist * dy, x + dist * dx]:
                            case "O":
                                continue
                            case "#":
                                crashed = True
                                break
                            case _:
                                crashed = False
                                break

                    if crashed:
                        continue

                    grid[y + dist * dy, x + dist * dx] = "O"
                case _:
                    pass

            grid[y, x] = "."
            x += dx
            y += dy
            grid[y, x] = "@"

        stones = numpy.where(grid == "O")

        return sum(100 * y + x for y, x in zip(*stones))

    @classmethod
    def part2(cls, input: str) -> None:
        input = input.replace(".", "..")
        input = input.replace("#", "##")
        input = input.replace("O", "[]")
        input = input.replace("@", "@.")

        grid, steps = parse_input(input)

        y, x = numpy.where(grid == "@")
        x, y = x[0], y[0]

        for c in steps:
            match c:
                case "^":
                    dx, dy = 0, -1
                case ">":
                    dx, dy = 1, 0
                case "<":
                    dx, dy = -1, 0
                case "v":
                    dx, dy = 0, 1
                case other:
                    raise ValueError(f"Invalid movement: {other}")

            match grid[y + dy, x + dx]:
                case "#":
                    continue
                case "]" | "[":
                    crashed = False
                    if dy == 0:
                        # easy case: just move linearly
                        for dist in itertools.count(2):
                            match grid[y, x + dist * dx]:
                                case "[" | "]":
                                    continue
                                case "#":
                                    crashed = True
                                    break
                                case _:
                                    break

                        if crashed:
                            continue

                        # shuffle all grid points one over
                        for steps in range(dist, 1, -1):
                            grid[y, x + dx * steps] = grid[y, x + dx * (steps - 1)]
                    else:
                        if grid[y + dy, x] == "[":
                            to_check = {x, x + 1}
                        else:
                            to_check = {x, x - 1}

                        moving_stones = [to_check]

                        for dist in itertools.count(2):
                            to_check_next = set()

                            for cx in to_check:
                                match grid[y + dist * dy, cx]:
                                    case "#":
                                        crashed = True
                                        break
                                    case "[":
                                        to_check_next.add(cx)
                                        to_check_next.add(cx + 1)
                                    case "]":
                                        to_check_next.add(cx)
                                        to_check_next.add(cx - 1)
                                    case _:
                                        continue

                            if crashed or not to_check_next:
                                break
                            moving_stones.append(to_check_next)
                            to_check = to_check_next

                        if crashed:
                            continue

                        for steps in range(len(moving_stones), 0, -1):
                            dist = steps + 1
                            for cx in moving_stones[steps - 1]:
                                grid[y + dy * dist, cx] = grid[y + dy * (dist - 1), cx]
                                grid[y + dy * (dist - 1), cx] = "."
                case _:
                    pass

            grid[y, x] = "."
            x += dx
            y += dy
            grid[y, x] = "@"

        stones = numpy.where(grid == "[")

        return sum(100 * y + x for y, x in zip(*stones))
