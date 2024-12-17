import re

from . import SeparateRunner


def run_program(
    register_a: int, register_b: int, register_c: int, program: list[int]
) -> list[int]:
    ip = 0
    out = []

    def combo(index: int) -> int:
        match program[index]:
            case 0:
                return 0
            case 1:
                return 1
            case 2:
                return 2
            case 3:
                return 3
            case 4:
                return register_a
            case 5:
                return register_b
            case 6:
                return register_c

    while ip < len(program):
        match program[ip]:
            case 0:  # adv
                register_a = register_a // 2 ** combo(ip + 1)
            case 1:  # bxl
                register_b ^= program[ip + 1]
            case 2:  # bst
                register_b = combo(ip + 1) & 0x7
            case 3:  # jnz
                if register_a != 0:
                    ip = program[ip + 1]
                    continue
            case 4:  # bxc
                register_b ^= register_c
            case 5:  # out
                out.append(combo(ip + 1) & 7)
            case 6:  # bdv
                register_b = register_a // 2 ** combo(ip + 1)
            case 7:  # cdv
                register_c = register_a // 2 ** combo(ip + 1)
        ip += 2

    return out


class DayRunner(SeparateRunner):
    @classmethod
    def part1(cls, input: str) -> str:
        numbers = re.findall(r"\d+", input)

        register_a, register_b, register_c = map(int, numbers[:3])
        program = list(map(int, numbers[3:]))

        out = run_program(register_a, register_b, register_c, program)

        return ",".join(map(str, out))

    @classmethod
    def part2(cls, input: str) -> int:
        numbers = re.findall(r"\d+", input)

        _, register_b, register_c = map(int, numbers[:3])
        program = list(map(int, numbers[3:]))

        cur = [0]

        # It came to me in a dream
        for entry in reversed(program):
            next_gen = []

            for num in cur:
                num *= 8
                for n in range(8):
                    output = run_program(num + n, register_b, register_c, program)
                    result = output[0]
                    if result == entry:
                        next_gen.append(num + n)

            cur = next_gen

        return cur[0]
