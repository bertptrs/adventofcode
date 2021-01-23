import itertools
from typing import List, TextIO, Tuple

from aoc2019.intcode import read_program, Computer


def amplify(phases: Tuple[int], program: List[int]) -> int:
    amps = []

    for i, phase in enumerate(phases):
        amp = Computer(program.copy())

        if i > 0:
            amp.input = amps[i - 1].output

        amp.input.append(phase)

        amps.append(amp)

    amps[0].input.append(0)

    for amp in amps:
        amp.run()

    return amps[-1].output.pop()


def reamplify(phases: Tuple[int], program: List[int]) -> int:
    amps = []

    for i, _ in enumerate(phases):
        amp = Computer(program.copy())

        if i > 0:
            amp.input = amps[i - 1].output

        amps.append(amp)

    amps[0].input = amps[-1].output

    for amp, phase in zip(amps, phases):
        amp.input.append(phase)

    amps[0].input.append(0)

    changes = True

    while changes:
        changes = False
        for amp in amps:
            try:
                amp.run()
            except IndexError:
                # Waiting for input
                changes = True

    return amps[-1].output.pop()


def part1(data: TextIO) -> int:
    program = read_program(data)

    return max(amplify(phase, program) for phase in itertools.permutations(range(0, 5)))


def part2(data: TextIO) -> int:
    program = read_program(data)

    return max(reamplify(phase, program) for phase in itertools.permutations(range(5, 10)))
