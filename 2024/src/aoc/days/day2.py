import numpy

from . import SeparateRunner


def _safe(nums: numpy.ndarray) -> bool:
    steps = nums[1:] - nums[:-1]

    if numpy.all(steps > 0):
        return numpy.all((steps >= 1) & (steps <= 3))
    elif numpy.all(steps < 0):
        return numpy.all((steps <= -1) & (steps >= -3))
    else:
        return False


def is_safe(line: str) -> bool:
    nums = numpy.fromstring(line, dtype=numpy.int32, sep=" ")

    return _safe(nums)


def is_savable(line: str) -> bool:
    nums = numpy.fromstring(line, dtype=numpy.int32, sep=" ")

    return any(
        _safe(numpy.concatenate((nums[:i], nums[i + 1 :]), axis=None))
        for i in range(len(nums))
    )


class DayRunner(SeparateRunner):
    @classmethod
    def part1(cls, data: str) -> int:
        lines = data.strip().split("\n")

        safe = sum(1 for line in lines if is_safe(line))

        return safe

    @classmethod
    def part2(cls, data: str) -> int:
        lines = data.strip().split("\n")

        safe = sum(1 for line in lines if is_savable(line))

        return safe
