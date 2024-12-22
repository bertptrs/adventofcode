from collections import defaultdict

import numpy
from numpy.lib.stride_tricks import sliding_window_view

from . import CombinedRunner


def advance(secrets: numpy.array) -> numpy.array:
    new_secrets = (secrets ^ (secrets << 6)) & 0xFFFFFF
    new_secrets ^= new_secrets >> 5
    new_secrets ^= new_secrets << 11
    new_secrets &= 0xFFFFFF

    return new_secrets


class DayRunner(CombinedRunner):
    @classmethod
    def run_both(cls, input: str) -> tuple[int, int]:
        secrets = numpy.fromstring(input, dtype=int, sep="\n")

        progression = [secrets]

        for _ in range(2000):
            secrets = advance(secrets)
            progression.append(secrets)

        field = numpy.stack(progression, axis=-1) % 10
        delta = field[:, 1:] - field[:, :-1]
        windows = sliding_window_view(delta, 4, axis=1)

        per_signal = defaultdict(int)

        for row_scores, row_deltas in zip(field, windows):
            unique, positions = numpy.unique(row_deltas, return_index=True, axis=0)

            for key, bananas in zip(unique, row_scores[positions + 4]):
                per_signal[tuple(key)] += bananas

        return secrets.sum(), max(per_signal.values())
