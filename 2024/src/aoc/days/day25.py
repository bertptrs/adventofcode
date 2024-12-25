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

        fitting = 0

        for key in keys:
            for lock in locks:
                if numpy.all((key + lock) <= 7):
                    fitting += 1

        return fitting, None
