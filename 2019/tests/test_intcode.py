from typing import List

import pytest

from aoc2019.intcode import Computer


@pytest.mark.parametrize('program,expected', [
    ([1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]),
    ([1, 0, 0, 0, 99], [2, 0, 0, 0, 99]),
    ([2, 3, 0, 3, 99], [2, 3, 0, 6, 99]),
    ([2, 4, 4, 5, 99, 0], [2, 4, 4, 5, 99, 9801]),
    ([1, 1, 1, 4, 99, 5, 6, 0, 99], [30, 1, 1, 4, 2, 5, 6, 0, 99]),
    # This is technically part of day 5 but it fits the pattern
    ([1002, 4, 3, 4, 33], [1002, 4, 3, 4, 99])
])
def test_instructions_day2(program: List[int], expected: List[int]) -> None:
    computer = Computer(program)

    computer.run()

    assert computer.program == expected
