from collections import defaultdict
from io import StringIO
from typing import Any

import numpy

from . import CombinedRunner


class DayRunner(CombinedRunner):
    @classmethod
    def run_both(cls, data: str) -> tuple[Any, Any]:
        nums = numpy.loadtxt(StringIO(data), dtype=numpy.int32)

        left = nums[..., 0]
        right = nums[..., 1]

        left.sort()
        right.sort()

        diff = numpy.abs(left - right).sum()

        counts: defaultdict[int, int] = defaultdict(int)
        for val in right:
            counts[val] += 1

        return diff, sum(counts[v] * v for v in left)
