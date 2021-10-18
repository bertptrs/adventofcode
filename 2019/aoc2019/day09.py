import sys
from typing import TextIO

from aoc2019.intcode import read_program, Computer


def run_machine(data: TextIO, initializer: int) -> int:
    program = read_program(data)
    computer = Computer(program)
    computer.input.append(initializer)

    computer.run()

    if len(computer.output) > 1:
        sys.exit(computer.output)
    else:
        return computer.output.pop()


def part1(data: TextIO) -> int:
    return run_machine(data, 1)


def part2(data: TextIO) -> int:
    return run_machine(data, 2)
