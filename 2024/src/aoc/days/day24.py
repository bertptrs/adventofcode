import functools
import re

from . import SeparateRunner


def parse_input(input: str) -> tuple[dict[str, int], dict[str, tuple[str, str, str]]]:
    variable_part, rules_part = input.strip().split("\n\n")

    variables = {}

    for line in variable_part.splitlines():
        variable, value = line.split(": ")
        variables[variable] = int(value)

    rules = {}

    for first, op, second, result in re.findall(
        r"(\w+) (XOR|OR|AND) (\w+) -> (\w+)", rules_part
    ):
        rules[result] = (first, op, second)

    return variables, rules


class DayRunner(SeparateRunner):
    @classmethod
    def part1(cls, input: str) -> int:
        variables, rules = parse_input(input)

        @functools.cache
        def get_value(variable: str) -> int:
            if variable in variables:
                return variables[variable]

            first, op, second = rules[variable]
            first_v = get_value(first)
            second_v = get_value(second)

            match op:
                case "AND":
                    return first_v & second_v
                case "OR":
                    return first_v | second_v
                case "XOR":
                    return first_v ^ second_v

        result = 0
        for variable in reversed(sorted(rules)):
            if not variable.startswith("z"):
                continue
            result = result * 2 + get_value(variable)

        return result

    @classmethod
    def part2(cls, input: str) -> str:
        variables, rules = parse_input(input)

        max_bit = int(
            max(variable for variable in rules if variable.startswith("z"))[1:]
        )

        def find_invalid(output: str, pattern) -> set[str]:
            if pattern is None:
                return set()

            if output in rules:
                left, op, right = rules[output]
            elif output == pattern:
                return set()
            else:
                return {output}

            pop, pleft, pright = pattern

            if op != pop:
                return {output}

            wrong_normal = find_invalid(left, pleft) | find_invalid(right, pright)
            wrong_mirror = find_invalid(left, pright) | find_invalid(right, pleft)

            least_wrong = min(wrong_mirror, wrong_normal, key=len)

            return least_wrong

        # First one is a half adder, that's a simple pattern
        invalid = find_invalid("z00", ["XOR", "x00", "y00"])
        # Second one is missing a reference to the before-previous adder, so it's a
        # slightly different patterns
        invalid |= find_invalid(
            "z01", ["XOR", ["AND", "x00", "y00"], ["XOR", "x01", "y01"]]
        )

        for n in range(2, max_bit):
            xcurr = f"x{n:02}"
            ycurr = f"y{n:02}"
            zcurr = f"z{n:02}"
            xprev = f"x{n-1:02}"
            yprev = f"y{n-1:02}"

            invalid |= find_invalid(
                zcurr,
                [
                    "XOR",
                    ["XOR", xcurr, ycurr],
                    ["OR", ["AND", xprev, yprev], ["AND", ["XOR", xprev, yprev], None]],
                ],
            )

        # This code somehow believes `ktp` is invalid, but it's fine on closer
        # inspection. Will figure that out later.

        return ",".join(sorted(invalid))
