from typing import Dict, Optional, TextIO

from aoc2019.intcode import Computer, read_program


def run_robot(data: TextIO, painted: Optional[Dict[complex, int]] = None) -> Dict[complex, int]:
    if painted is None:
        painted = {}

    computer = Computer(read_program(data))

    pos = 0j
    direction = 1j

    finished = False

    while not finished:
        try:
            computer.run()
            finished = True
        except IndexError:
            pass

        while len(computer.output) >= 2:
            paint = computer.output.popleft()
            turn = computer.output.popleft()

            painted[pos] = paint

            if turn:
                direction *= -1j
            else:
                direction *= 1j

            pos += direction

        computer.input.append(painted.get(pos, 0))

    return painted


def part1(data: TextIO) -> int:
    return len(run_robot(data))


def part2(data: TextIO) -> str:
    painted = run_robot(data, {0j: 1})

    xmin = int(min(pos.real for pos in painted.keys()))
    xmax = int(max(pos.real for pos in painted.keys()))
    ymin = int(min(pos.imag for pos in painted.keys()))
    ymax = int(max(pos.imag for pos in painted.keys()))

    image = ''

    for y in reversed(range(ymin, ymax + 1)):
        line = ''.join('#' if painted.get(x + y * 1j) == 1 else ' ' for x in range(xmin, xmax + 1))
        image += f'{line}\n'

    return image[:-1]
