from . import SeparateRunner


def parse_input(input: str) -> tuple[int, list[int]]:
    result = []

    for line in input.strip().split("\n"):
        test, nums = line.split(": ")
        result.append((int(test), list(map(int, nums.split(" ")))))

    return result


def is_possible(target: int, nums: list[int], cur: int) -> bool:
    if cur == target and not nums:
        return True

    if cur > target or not nums:
        return False

    head = nums[0]
    remainder = nums[1:]

    return is_possible(target, remainder, cur + head) or is_possible(
        target, remainder, cur * head
    )


def is_possible2(target: int, nums: list[int], cur: int) -> bool:
    if cur == target and not nums:
        return True

    if cur > target or not nums:
        return False

    head = nums[0]
    remainder = nums[1:]

    return (
        is_possible2(target, remainder, cur + head)
        or is_possible2(target, remainder, cur * head)
        or is_possible2(target, remainder, int(f"{cur}{head}"))
    )


class DayRunner(SeparateRunner):
    @classmethod
    def part1(cls, input: str) -> int:
        lines = parse_input(input)

        return sum(
            target for target, nums in lines if is_possible(target, nums[1:], nums[0])
        )

    @classmethod
    def part2(cls, input: str) -> int:
        lines = parse_input(input)

        return sum(
            target for target, nums in lines if is_possible2(target, nums[1:], nums[0])
        )
