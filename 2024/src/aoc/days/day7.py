from . import SeparateRunner


def parse_input(input: str) -> tuple[int, list[int]]:
    result = []

    for line in input.strip().split("\n"):
        test, nums = line.split(": ")
        result.append((int(test), list(map(int, nums.split(" ")))))

    return result


def is_possible(target: int, nums: list[int]) -> bool:
    if target == 0:
        return not nums
    if not nums or target < 0:
        return False

    tail = nums[-1]
    remainder = nums[:-1]

    return is_possible(target - tail, remainder) or (
        target % tail == 0 and is_possible(target // tail, remainder)
    )


def is_possible2(target: int, nums: list[int]) -> bool:
    if target == 0:
        return not nums
    if not nums or target < 0:
        return False

    tail = nums[-1]
    remainder = nums[:-1]

    target_str = str(target)
    tail_str = str(tail)

    return (
        is_possible2(target - tail, remainder)
        or (target % tail == 0 and is_possible2(target // tail, remainder))
        or (
            target_str.endswith(tail_str)
            and is_possible2(int(target_str[: -len(str(tail_str))]), remainder)
        )
    )


class DayRunner(SeparateRunner):
    @classmethod
    def part1(cls, input: str) -> int:
        lines = parse_input(input)

        return sum(target for target, nums in lines if is_possible(target, nums))

    @classmethod
    def part2(cls, input: str) -> int:
        lines = parse_input(input)

        return sum(target for target, nums in lines if is_possible2(target, nums))
