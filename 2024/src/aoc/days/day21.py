import functools
from collections import Counter, defaultdict

from . import SeparateRunner

NUMPAD = {
    "A": (3, 2),
    "0": (3, 1),
    "1": (2, 0),
    "2": (2, 1),
    "3": (2, 2),
    "4": (1, 0),
    "5": (1, 1),
    "6": (1, 2),
    "7": (0, 0),
    "8": (0, 1),
    "9": (0, 2),
}

DIRPAD = {
    "A": (0, 2),
    "^": (0, 1),
    "<": (1, 0),
    "v": (1, 1),
    ">": (1, 2),
}


@functools.cache
def shortest_numpad(from_: str, to: str) -> list[str]:
    inverse = set(NUMPAD.values())
    ay, ax = NUMPAD[from_]
    by, bx = NUMPAD[to]

    dx, dy = bx - ax, by - ay

    sx = "<" if dx < 0 else ">"
    sy = "^" if dy < 0 else "v"

    if dx > 0 and (by, ax) in inverse or (ay, bx) not in inverse:
        return abs(dy) * sy + abs(dx) * sx + "A"
    else:
        return abs(dx) * sx + abs(dy) * sy + "A"


@functools.cache
def shortest_dirpad(from_: str, to: str) -> str:
    inverse = set(DIRPAD.values())
    ay, ax = DIRPAD[from_]
    by, bx = DIRPAD[to]

    dx, dy = bx - ax, by - ay
    sx = "<" if dx < 0 else ">"
    sy = "^" if dy < 0 else "v"

    if dx > 0 and (by, ax) in inverse or (ay, bx) not in inverse:
        return abs(dy) * sy + abs(dx) * sx + "A"
    else:
        return abs(dx) * sx + abs(dy) * sy + "A"


def encode_shortest_numpad(code: str) -> str:
    pos = "A"

    res = ""

    for c in code:
        res += shortest_numpad(pos, c)
        # print(c, res)
        pos = c

    return res


def encode_shortest_dirpad(code: str) -> str:
    pos = "A"

    res = ""

    for c in code:
        if pos != c:
            res += shortest_dirpad(pos, c)
        else:
            res += "A"
        pos = c

    return res


def decode(code: str, pad: dict[str, tuple[int, int]]) -> str:
    result = ""
    inverse = {v: k for k, v in pad.items()}

    y, x = pad["A"]

    for i, c in enumerate(code):
        match c:
            case "A":
                result += inverse[y, x]
            case "^":
                y -= 1
            case "v":
                y += 1
            case "<":
                x -= 1
            case ">":
                x += 1

        if (y, x) not in inverse:
            raise ValueError(
                f"""Moved off the board {x, y}, after processing {c}.
                Path so far: {result} (from {code[:i]})"""
            )

    return result


def count_steps(path: str, count: int) -> dict[str, int]:
    cur = "A"
    counts = defaultdict(int)

    for c in path:
        step = shortest_dirpad(cur, c)
        cur = c
        counts[step] += count

    return counts


class DayRunner(SeparateRunner):
    @classmethod
    def part1(cls, input: str) -> int:
        result = 0
        for code in input.strip().split("\n"):
            numpad = encode_shortest_numpad(code)
            robot1 = encode_shortest_dirpad(numpad)
            robot2 = encode_shortest_dirpad(robot1)

            result += int(code[:-1]) * len(robot2)

        return result

    @classmethod
    def part2(cls, input: str, robots=25) -> int:
        result = 0
        for code in input.strip().split("\n"):
            numpad = encode_shortest_numpad(code)
            keypresses = Counter([numpad])

            for _ in range(robots + 1):
                new_presses = Counter()
                for subroute, count in keypresses.items():
                    new_presses.update(count_steps(subroute, count))

                keypresses = new_presses

            result += int(code[:-1]) * keypresses.total()

        return result
