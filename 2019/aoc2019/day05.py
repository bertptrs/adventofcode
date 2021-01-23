from typing import TextIO

from aoc2019.intcode import read_program, Computer


def part1(data: TextIO) -> int:
    program = read_program(data)

    computer = Computer(program)

    # Enter the required starting code
    computer.input.append(1)

    computer.run()

    return computer.output.pop()


def part2(data: TextIO) -> int:
    program = read_program(data)

    computer = Computer(program)

    # Enter the required starting code
    computer.input.append(5)

    computer.run()

    return computer.output.pop()
