import heapq

from . import SeparateRunner


def file_checksum(file_id: int, start: int, length: int) -> int:
    return file_id * length * (2 * start + length - 1) // 2


class DayRunner(SeparateRunner):
    @classmethod
    def part1(cls, input: str) -> int:
        files = []
        empty = []

        pos = 0

        for c in input.strip():
            val = int(c)

            if len(files) == len(empty):
                files.append((pos, val))
            else:
                empty.append((pos, val))

            pos += val

        checksum = 0

        for start, length in empty:
            while files and length > 0:
                file_start, file_len = files.pop()
                if file_start < start:
                    files.append((file_start, file_len))
                    break

                file_id = len(files)

                infill = min(file_len, length)

                checksum += file_checksum(file_id, start, infill)
                start += infill

                if infill != file_len:
                    files.append((file_start, file_len - infill))

                length -= infill
            else:
                continue
            break

        for file_id, (file_start, file_len) in enumerate(files):
            checksum += file_checksum(file_id, file_start, file_len)

        return checksum

    @classmethod
    def part2(cls, input: str) -> int:
        files = []
        empty = [[] for _ in range(10)]

        pos = 0

        is_file = True

        for c in input.strip():
            val = int(c)

            if is_file:
                files.append((pos, val))
                is_file = False
            else:
                # No need for heappush, as we're appending values in order
                empty[val].append(pos)
                is_file = True

            pos += val

        checksum = 0

        while files:
            start, length = files.pop()
            file_id = len(files)

            best = None
            best_heap = None

            for i, heap in enumerate(empty[length:]):
                if not heap or heap[0] > start:
                    continue

                if best is None or best > heap[0]:
                    best = heap[0]
                    best_heap = i + length

            if best is None:
                # No room to move left, count score at current position
                checksum += file_checksum(file_id, start, length)
            else:
                checksum += file_checksum(file_id, best, length)
                heapq.heappop(empty[best_heap])

                if length < best_heap:
                    remainder = best_heap - length
                    heapq.heappush(empty[remainder], best + length)

        return checksum
