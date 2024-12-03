import re

from . import SeparateRunner


class DayRunner(SeparateRunner):
    @classmethod
    def part1(cls, data: str) -> int:
        return sum(int(a) * int(b) for a, b in re.findall(r"mul\((\d+),(\d+)\)", data))

    @classmethod
    def part2(cls, data: str) -> int:
        do = True
        total = 0
        for op, a, b in re.findall(r"(don't\(\)|do\(\)|mul\((\d+),(\d+)\))", data):
            match op:
                case "do()":
                    do = True
                case "don't()":
                    do = False
                case _:
                    if do:
                        total += int(a) * int(b)

        return total
