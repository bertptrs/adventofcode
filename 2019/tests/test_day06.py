from io import StringIO

from aoc2019.day06 import part1, part2


def test_sample_part1():
    data = StringIO("""\
            COM)B
            B)C
            C)D
            D)E
            E)F
            B)G
            G)H
            D)I
            E)J
            J)K
            K)L""")

    assert part1(data) == 42


def test_sample_part2():
    data = StringIO("""\
            COM)B
            B)C
            C)D
            D)E
            E)F
            B)G
            G)H
            D)I
            E)J
            J)K
            K)L
            K)YOU
            I)SAN""")

    assert part2(data) == 4
