from io import StringIO
import textwrap

import pytest


from aoc2019.day18 import part1

SAMPLES = list(
    map(
        textwrap.dedent,
        [
            """
            #########
            #b.A.@.a#
            #########
            """,
            """
            ########################
            #f.D.E.e.C.b.A.@.a.B.c.#
            ######################.#
            #d.....................#
            ########################
            """,
            """
            ########################
            #...............b.C.D.f#
            #.######################
            #.....@.a.B.c.d.A.e.F.g#
            ########################
            """,
            """
            #################
            #i.G..c...e..H.p#
            ########.########
            #j.A..b...f..D.o#
            ########@########
            #k.E..a...g..B.n#
            ########.########
            #l.F..d...h..C.m#
            #################
            """,
            """
            ########################
            #@..............ac.GI.b#
            ###d#e#f################
            ###A#B#C################
            ###g#h#i################
            ########################
            """,
        ],
    )
)


@pytest.mark.parametrize(
    "map_id, shortest", list(zip(range(len(SAMPLES)), [8, 86, 132, 136, 81]))
)
def test_sample_part1(map_id: int, shortest: int):
    data = StringIO(SAMPLES[map_id])
    result = part1(data)

    assert result == shortest
