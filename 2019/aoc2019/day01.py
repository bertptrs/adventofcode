from typing import TextIO


def fuel_required(weight: int) -> int:
    return max(0, weight // 3 - 2)


def recursive_fuel_required(weight: int) -> int:
    total = 0

    while weight > 0:
        weight = fuel_required(weight)
        total += weight

    return total


def part1(data: TextIO) -> int:
    return sum(fuel_required(int(line)) for line in data)


def part2(data: TextIO) -> int:
    return sum(recursive_fuel_required(int(line)) for line in data)
