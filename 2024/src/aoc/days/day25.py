import numpy

from . import CombinedRunner


class DayRunner(CombinedRunner):
    @classmethod
    def run_both(cls, input: str) -> tuple[int, None]:
        blocks = input.strip().split("\n\n")

        keys = []
        locks = []

        for block in blocks:
            grid = numpy.array(list(map(list, block.splitlines())))
            heights = numpy.count_nonzero(grid == "#", axis=0)

            if block.startswith("#####"):
                locks.append(heights)
            else:
                keys.append(heights)

        locks = numpy.stack(locks, axis=0)

        fitting = sum(
            numpy.count_nonzero(numpy.all((key + locks) <= 7, axis=1)) for key in keys
        )

        return fitting, None
