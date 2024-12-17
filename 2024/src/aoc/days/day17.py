import re

from . import SeparateRunner


class DayRunner(SeparateRunner):
    @classmethod
    def part1(cls, input: str) -> str:
        numbers = re.findall(r"\d+", input)

        register_a, register_b, register_c = map(int, numbers[:3])
        program = list(map(int, numbers[3:]))

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

        return ",".join(map(str, out))

    @classmethod
    def part2(cls, input: str) -> str:
        pass
