import importlib
from abc import ABC, abstractmethod
from typing import Any, Tuple, cast


class Runner(ABC):
    @classmethod
    @abstractmethod
    def run_both(cls, data: str) -> Tuple[Any, Any]:
        pass

    @classmethod
    @abstractmethod
    def part1(cls, data: str) -> Any:
        pass

    @classmethod
    @abstractmethod
    def part2(cls, data: str) -> Any:
        pass


class SeparateRunner(Runner):
    @classmethod
    def run_both(cls, data: str) -> Tuple[Any, Any]:
        return (cls.part1(data), cls.part2(data))


class CombinedRunner(Runner):
    @classmethod
    def part1(cls, data: str) -> Any:
        return cls.run_both(data)[0]

    @classmethod
    def part2(cls, data: str) -> Any:
        return cls.run_both(data)[1]


def get_runner(day: int) -> type[Runner]:
    runner_module = importlib.import_module(f".day{day}", package=__name__)

    assert issubclass(runner_module.DayRunner, Runner)

    return cast(type[Runner], runner_module.DayRunner)
