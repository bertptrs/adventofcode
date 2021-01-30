import statistics
from typing import TextIO, Tuple, Dict

from aoc2019.intcode import Computer, read_program


def render_screen(computer: Computer, screen: Dict[Tuple[int, int], int]):
    while computer.output:
        x = computer.output.popleft()
        y = computer.output.popleft()
        val = computer.output.popleft()

        screen[x, y] = val


def part1(data: TextIO) -> int:
    computer = Computer(read_program(data))
    computer.run()

    screen: Dict[Tuple[int, int], int] = {}

    render_screen(computer, screen)

    return sum(1 for val in screen.values() if val == 2)


def part2(data: TextIO) -> int:
    computer = Computer(read_program(data))

    computer.program[0] = 2

    screen: Dict[Tuple[int, int], int] = {}

    finished = False

    while not finished:
        try:
            computer.run()
            finished = True
        except IndexError:
            # Waiting for input
            pass

        render_screen(computer, screen)

        ball_x = next(x for x, y in screen if screen[x, y] == 4)

        paddle_x = statistics.mean(x for x, y in screen if screen[x, y] == 3)

        if ball_x < paddle_x:
            computer.input.append(-1)
        elif ball_x > paddle_x:
            computer.input.append(1)
        else:
            computer.input.append(0)

    return screen[-1, 0]
