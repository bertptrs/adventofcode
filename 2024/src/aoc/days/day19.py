import functools

from . import CombinedRunner


def parse_input(data: str) -> tuple[tuple[str, ...], list[str]]:
    patterns, designs = data.strip().split("\n\n")

    return tuple(patterns.split(", ")), designs.split("\n")


class DayRunner(CombinedRunner):
    @classmethod
    def run_both(cls, input: str) -> int:
        patterns, designs = parse_input(input)

        possible = 0
        ways = 0

        @functools.cache
        def is_possible(design: str) -> bool:
            if not design:
                return 1

            return sum(
                is_possible(design[len(pat) :])
                for pat in patterns
                if design.startswith(pat)
            )

        for design in designs:
            if (solve := is_possible(design)) > 0:
                possible += 1
                ways += solve

        return possible, ways
