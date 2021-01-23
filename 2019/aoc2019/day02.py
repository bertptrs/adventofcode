from typing import TextIO

from aoc2019.intcode import read_program, Computer


def part1(data: TextIO) -> int:
    program = read_program(data)

    program[1] = 12
    program[2] = 2

    computer = Computer(program)
    computer.run()

    return computer[0]


def part2(data: TextIO) -> int:
    program = read_program(data)

    for verb in range(100):
        for noun in range(100):
            computer = Computer(program.copy())

            computer[1] = noun
            computer[2] = verb

            computer.run()

            if computer[0] == 19690720:
                return 100 * noun + verb

    raise ValueError('Did not find valid combination')
