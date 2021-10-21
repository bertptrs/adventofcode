import copy
from itertools import product
from typing import TextIO

from aoc2019.intcode import Computer, read_program


def query_position(x: int, y: int, computer: Computer) -> bool:
    computer = copy.deepcopy(computer)

    computer.send_input(x)
    computer.send_input(y)
    computer.run()

    return computer.get_output() == 1


def part1(data: TextIO) -> int:
    computer = Computer(read_program(data))

    return sum(1 for x, y in product(range(50), range(50)) if query_position(x, y, computer))
