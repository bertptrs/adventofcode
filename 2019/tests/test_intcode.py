import itertools
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


@pytest.mark.parametrize('number,program', itertools.product([7, 8, 9], [
    [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
    [3, 3, 1108, -1, 8, 3, 4, 3, 99],
]))
def test_equality_opcode(program: List[int], number: int):
    computer = Computer(program.copy())
    computer.input.append(number)

    computer.run()

    assert computer.output.pop() == int(number == 8)


@pytest.mark.parametrize('number,program', itertools.product([7, 8, 9], [
    [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
    [3, 3, 1107, -1, 8, 3, 4, 3, 99],
]))
def test_less_than_opcode(program: List[int], number: int):
    computer = Computer(program.copy())
    computer.input.append(number)

    computer.run()

    assert computer.output.pop() == int(number < 8)


@pytest.mark.parametrize('inputs,expected', [
    (12, 1001),
    (8, 1000),
    (2, 999),
])
def test_day5_example(inputs: int, expected: int):
    computer = Computer([3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
                         1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
                         999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99])

    computer.input.append(inputs)

    computer.run()

    assert computer.output.pop() == expected


@pytest.mark.parametrize('program,output', [
    ([109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99],
     [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]),
    ([1102, 34915192, 34915192, 7, 4, 7, 99, 0], [1219070632396864]),
    ([104, 1125899906842624, 99], [1125899906842624]),
])
def test_instructions_day9(program: List[int], output: List[int]) -> None:
    computer = Computer(program)
    computer.run()

    result = list(computer.output)

    assert result == output
