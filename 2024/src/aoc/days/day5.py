from collections import defaultdict
import functools

from . import CombinedRunner


def parse_input(input: str) -> tuple[set[tuple[int, int]], list[list[int]]]:
    first, second = input.strip().split("\n\n")

    rules = {tuple(int(x) for x in line.split("|")) for line in first.split("\n")}
    updates = [[int(x) for x in line.split(",")] for line in second.split("\n")]

    return rules, updates


def is_correct(update: list[int], must_after: dict[int, set[int]]) -> bool:
    forbidden = set()

    for entry in update:
        if entry in forbidden:
            return False

        forbidden |= must_after.get(entry, set())
        
    return True


class DayRunner(CombinedRunner):
    @classmethod
    def run_both(cls, input: str) -> int:
        rules, updates = parse_input(input)

        must_after = defaultdict(set)

        for before, after in rules:
            must_after[after].add(before)

        correct = 0
        corrected = 0

        key = functools.cmp_to_key(lambda a, b: -1 if (a, b) in rules else 1)

        for update in updates:
            if is_correct(update, must_after):
                correct += update[len(update) // 2]
            else:
                update.sort(key=key)
                corrected += update[len(update) // 2]

        return correct, corrected
