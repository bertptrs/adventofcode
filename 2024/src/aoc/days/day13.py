import re

import numpy

from . import CombinedRunner

NASTY_REGEX = r"""Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)"""


class DayRunner(CombinedRunner):
    @classmethod
    def run_both(cls, input: str) -> tuple[int, int]:
        machines = re.findall(NASTY_REGEX, input)

        cost_to_win = 0
        cost_to_win2 = 0
        scale = 10000000000000

        for machine in machines:
            ax, ay, bx, by, px, py = map(int, machine)

            X = numpy.array([[ax, bx], [ay, by]])
            B = numpy.array([px, py])
            B2 = numpy.array([px + scale, py + scale])

            A = numpy.linalg.solve(X, B)
            A2 = numpy.linalg.solve(X, B2)

            a_press, b_press = map(round, A)
            a_press2, b_press2 = map(round, A2)

            if a_press * ax + b_press * bx == px and a_press * ay + b_press * by == py:
                cost_to_win += 3 * a_press + b_press

            if (
                a_press2 * ax + b_press2 * bx == px + scale
                and a_press2 * ay + b_press2 * by == py + scale
            ):
                cost_to_win2 += 3 * a_press2 + b_press2

        return cost_to_win, cost_to_win2
