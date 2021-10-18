import itertools
import math
import re
from typing import TextIO

import numpy  # type: ignore


def read_moons(data: TextIO) -> numpy.array:
    moons = []

    for line in data:
        moon = [int(x) for x in re.split(r"[^-0-9]+", line) if x]
        moons.append(moon)

    return numpy.array(moons)


def advance(pos: numpy.array, vel: numpy.array) -> None:
    """ update pos and vel in place """
    pos_prime = numpy.repeat(numpy.reshape(pos, (1, len(pos))), len(pos), axis=0).transpose() - pos
    pos_prime = -numpy.sign(pos_prime)
    vel += numpy.sum(pos_prime, axis=1)
    pos += vel


def simulate_moons(moons: numpy.array, iterations: int) -> int:
    moons = numpy.transpose(moons)  # Transpose so we have rows of x, y, z
    velocity = numpy.zeros_like(moons)

    for _ in range(iterations):
        for pos, vel in zip(moons, velocity):
            advance(pos, vel)

    potential = numpy.sum(numpy.abs(moons), axis=0)
    kinetic = numpy.sum(numpy.abs(velocity), axis=0)

    return int(numpy.sum(kinetic * potential))


def find_repetition(moons: numpy.array) -> int:
    moons = numpy.transpose(moons)
    velocity = numpy.zeros_like(moons)

    needed = 1

    for pos, vel in zip(moons, velocity):
        pos_prime = numpy.copy(pos)

        for i in itertools.count(1):
            advance(pos, vel)

            if (pos == pos_prime).all() and (vel == 0).all():
                needed *= i // math.gcd(needed, i)
                break

    return needed


def part1(data: TextIO) -> int:
    moons = read_moons(data)

    return simulate_moons(moons, 1000)


def part2(data: TextIO) -> int:
    moons = read_moons(data)

    return find_repetition(moons)
