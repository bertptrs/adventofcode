from typing import TextIO

from aoc2019.intcode import Computer, read_program


def part1(data: TextIO) -> int:
    computer = Computer(read_program(data))

    computer.run()

    output = ''.join(chr(c) for c in computer.output)

    tiles = set()

    for y, line in enumerate(output.splitlines()):
        for x, c in enumerate(line):
            if c == '#':
                tiles.add((x, y))

    total = 0

    for x, y in tiles:
        if (x - 1, y) in tiles and (x + 1, y) in tiles and (x, y - 1) in tiles and (x, y + 1) in tiles:
            total += x * y

    return total
