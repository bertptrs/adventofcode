from typing import TextIO

from aoc2019.intcode import Computer, read_program


def part1(data: TextIO) -> int:
    computer = Computer(read_program(data))

    computer.run()

    screen = {}

    while computer.output:
        x = computer.output.popleft()
        y = computer.output.popleft()
        val = computer.output.popleft()

        screen[x, y] = val

    return sum(1 for val in screen.values() if val == 2)
