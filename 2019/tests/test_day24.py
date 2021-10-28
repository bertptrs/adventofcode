import io

import pytest

from aoc2019.day24 import read_board, advance_board, part1

SAMPLE_START = """\
....#
#..#.
#..##
..#..
#....
"""

SAMPLE_STATES = """\
....#
#..#.
#..##
..#..
#....

#..#.
####.
###.#
##.##
.##..

#####
....#
....#
...#.
#.###

#....
####.
...##
#.##.
.##.#

####.
....#
##..#
.....
##...
""".split("\n\n")


@pytest.mark.parametrize("cycles,state", enumerate(SAMPLE_STATES))
def test_evolution_part1(cycles: int, state: str) -> None:
    with io.StringIO(SAMPLE_START) as f:
        board = read_board(f)

    with io.StringIO(state) as f:
        final_state = read_board(f)

    for _ in range(cycles):
        board = advance_board(board)

    assert board == final_state


def test_sample_part1() -> None:
    with io.StringIO(SAMPLE_START) as f:
        assert part1(f) == 2129920
