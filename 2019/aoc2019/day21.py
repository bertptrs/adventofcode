from typing import TextIO

from aoc2019.intcode import read_program, Computer


def send_input(computer: Computer, program: str) -> None:
    for c in program:
        computer.send_input(ord(c))


def run(data: TextIO, program: str) -> int:
    computer = Computer(read_program(data))

    send_input(computer, program)
    computer.run()

    return computer.output.pop()


def part1(data: TextIO) -> int:
    program = """\
    OR A J
    AND B J
    AND C J
    NOT J J
    AND D J
    WALK
    """

    return run(data, program)


def part2(data: TextIO) -> int:
    program = """\
    NOT H J
    OR C J
    AND A J
    AND B J
    NOT J J
    AND D J
    RUN
    """

    return run(data, program)
