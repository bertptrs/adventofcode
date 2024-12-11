import functools
from collections import defaultdict

from . import CombinedRunner


@functools.cache
def blink_num(num: int) -> tuple[int, ...]:
    if num == 0:
        return (1,)

    num_str = str(num)
    num_len = len(num_str)

    if num_len % 2 == 0:
        half = num_len // 2
        return (int(num_str[:half]), int(num_str[half:]))

    return (num * 2024,)


def step(nums: dict[int, int]) -> dict[int, int]:
    result = defaultdict(int)

    for num, count in nums.items():
        for transformed in blink_num(num):
            result[transformed] += count

    return result


class DayRunner(CombinedRunner):
    @classmethod
    def run_both(cls, input: str) -> tuple[int, int]:
        nums = [int(val) for val in input.strip().split(" ")]

        counts = defaultdict(int)

        for num in nums:
            counts[num] += 1

        for _ in range(25):
            counts = step(counts)

        part1 = sum(counts.values())

        for _ in range(50):
            counts = step(counts)

        return part1, sum(counts.values())
