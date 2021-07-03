import pytest

from aoc2019.day22 import shuffle

SAMPLE_INSTRUCTIONS = [
    """deal with increment 7
    deal into new stack
    deal into new stack""",
    """cut 6
    deal with increment 7
    deal into new stack""",
    """deal with increment 7
    deal with increment 9
    cut -2""",
    """deal into new stack
    cut -2
    deal with increment 7
    cut 8
    cut -4
    deal with increment 7
    cut 3
    deal with increment 9
    deal with increment 3
    cut -1""",
]

CORRECT_SHUFFLES = [
    "0 3 6 9 2 5 8 1 4 7",
    "3 0 7 4 1 8 5 2 9 6",
    "6 3 0 7 4 1 8 5 2 9",
    "9 2 5 8 1 4 7 0 3 6",
]


@pytest.mark.parametrize('instructions,correct', zip(SAMPLE_INSTRUCTIONS, CORRECT_SHUFFLES))
def test_shuffle(instructions, correct):
    instructions = [line.strip() for line in instructions.splitlines()]

    correct = [int(i) for i in correct.split(" ")]

    result = shuffle(instructions, 10)

    assert result == correct
