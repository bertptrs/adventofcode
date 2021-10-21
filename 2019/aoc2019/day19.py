import copy
from itertools import product
from typing import TextIO, Tuple

from aoc2019.intcode import Computer, read_program


def query_position(x: int, y: int, computer: Computer) -> bool:
    computer = copy.deepcopy(computer)

    computer.send_input(x)
    computer.send_input(y)
    computer.run()

    return computer.get_output() == 1


def find_line(y: int, x_min: int, x_max: int, computer: Computer) -> Tuple[int, int]:
    # First find start of the line:
    offset = 0

    while not query_position(x_min, y, computer):
        offset += 1
        x_min += 1

    x_max += offset
    while query_position(x_max, y, computer):
        x_max += 1

    x_max -= 1

    return x_min, x_max


def part1(data: TextIO) -> int:
    computer = Computer(read_program(data))

    x_min, x_max = (0, 0)
    total = 0

    for y in range(50):
        x_min, x_max = find_line(y, x_min, x_max, computer)
        total += min(x_max, 49) - min(x_min, 50) + 1

    return total
