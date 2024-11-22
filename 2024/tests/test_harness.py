from aoc import days


def test_harness_works() -> None:
    runner = days.get_runner(1)
    assert runner is not None
