from collections import defaultdict

import numpy
from numpy.lib.stride_tricks import sliding_window_view

from . import SeparateRunner


def advance(secrets: numpy.array) -> numpy.array:
    new_secrets = (secrets ^ (secrets << 6)) & 0xFFFFFF
    new_secrets ^= new_secrets >> 5
    new_secrets ^= new_secrets << 11
    new_secrets &= 0xFFFFFF

    return new_secrets


class DayRunner(SeparateRunner):
    @classmethod
    def part1(cls, input: str) -> int:
        secrets = numpy.fromstring(input, dtype=int, sep="\n")

        for _ in range(2000):
            secrets = advance(secrets)

        return secrets.sum()

    @classmethod
    def part2(cls, input: str) -> int:
        secrets = numpy.fromstring(input, dtype=int, sep="\n")

        progression = [secrets]

        for _ in range(2000):
            secrets = advance(secrets)
            progression.append(secrets)

        field = numpy.stack(progression, axis=-1) % 10
        delta = field[:, 1:] - field[:, :-1]

        per_signal = defaultdict(int)

        for row_scores, row_deltas in zip(field, delta):
            unique, positions = numpy.unique(
                sliding_window_view(row_deltas, 4), return_index=True, axis=0
            )

            for key, index in zip(unique, positions):
                per_signal[tuple(key)] += row_scores[index + 4]

        return max(per_signal.values())
