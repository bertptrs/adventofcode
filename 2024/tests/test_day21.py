import pytest

from aoc.days.day21 import (
    DayRunner,
    encode_shortest_dirpad,
    encode_shortest_numpad,
)

from . import get_data


def test_encode_shortest_numpad() -> None:
    assert encode_shortest_numpad("029A") in (
        "<A^A>^^AvvvA",
        "<A^A^>^AvvvA",
        "<A^A^^>AvvvA",
    )


def test_encode_shortest_dirpad() -> None:
    numpad_encoded = encode_shortest_numpad("029A")
    assert len(encode_shortest_dirpad(numpad_encoded)) == len(
        "v<<A>>^A<A>AvA<^AA>A<vAAA>^A"
    )


@pytest.mark.parametrize(
    "code,answer",
    [
        (
            "029A",
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
        ),
        ("980A", "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A"),
        (
            "179A",
            "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
        ),
        ("456A", "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A"),
        ("379A", "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"),
    ],
)
def test_encode_shortest_dirpad_twice(code: str, answer: str) -> None:
    numpad_encoded = encode_shortest_numpad(code)
    robot1 = encode_shortest_dirpad(numpad_encoded)
    robot2 = encode_shortest_dirpad(robot1)
    assert len(robot2) == len(answer)


def test_sample_part1() -> None:
    assert DayRunner.part1(get_data(21)) == 126384
