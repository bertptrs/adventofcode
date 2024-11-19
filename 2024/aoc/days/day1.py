from typing import Any

from . import SeparateRunner


class DayRunner(SeparateRunner):
    @classmethod
    def part1(cls, _data: str) -> Any:
        return "Hello"

    @classmethod
    def part2(cls, _data: str) -> Any:
        return "world!"
