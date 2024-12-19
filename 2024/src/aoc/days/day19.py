import collections
import functools

from . import CombinedRunner


def parse_input(data: str) -> tuple[list[str], list[str]]:
    patterns, designs = data.strip().split("\n\n")

    return patterns.split(", "), designs.split("\n")


class DayRunner(CombinedRunner):
    @classmethod
    def run_both(cls, input: str) -> int:
        patterns, designs = parse_input(input)

        by_prefix = collections.defaultdict(list)
        for prefix in patterns:
            by_prefix[prefix[0]].append(prefix)

        possible = 0
        ways = 0

        @functools.cache
        def is_possible(design: str) -> bool:
            if not design:
                return 1
            else:
                return sum(
                    is_possible(design[len(prefix) :])
                    for prefix in by_prefix[design[0]]
                    if design.startswith(prefix)
                )

        for design in designs:
            if (solve := is_possible(design)) > 0:
                possible += 1
                ways += solve

        return possible, ways
