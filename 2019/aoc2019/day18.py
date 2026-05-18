from collections import deque
import heapq
from typing import TextIO


def bfs(
    start: tuple[int, int], map_: list[str]
) -> list[tuple[str, int, tuple[frozenset]]]:
    visited = {start}
    todo = deque([(start, 0, frozenset())])

    result = []

    while len(todo) > 0:
        pos, dist, keys = todo.popleft()
        x, y = pos

        for next_ in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]:
            if next_ in visited:
                continue

            visited.add(next_)

            match map_[next_[1]][next_[0]]:
                case "#":
                    continue
                case letter if letter.islower():
                    result.append((letter, dist + 1, keys))
                case other:
                    if other.isupper():
                        next_keys = keys.union({other.lower()})
                    else:
                        next_keys = keys

                    todo.append((next_, dist + 1, next_keys))

    return result


def part1(data: TextIO) -> int:
    map_ = data.read().strip().splitlines()

    keys = {}

    for y, line in enumerate(map_):
        for x, c in enumerate(line):
            match c:
                case "@":
                    keys["@"] = (x, y)
                case key if key.islower():
                    keys[key] = (x, y)
                case _:
                    continue

    key_to_keys = {key: bfs(pos, map_) for key, pos in keys.items()}
    target_keys = len(keys) - 1

    todo = [(0, "@", frozenset())]
    shortest = {}

    while len(todo) > 0:
        dist, pos, keys = heapq.heappop(todo)
        if len(keys) == target_keys:
            return dist

        for next_, next_dist, required_keys in key_to_keys[pos]:
            if not required_keys.issubset(keys):
                continue

            next_total = dist + next_dist
            next_keys = keys.union({next_})
            next_state = (next_, next_keys)

            if next_state not in shortest or shortest[next_state] > next_total:
                shortest[next_state] = next_total
                heapq.heappush(todo, (next_total, next_, next_keys))

    raise ValueError("Did not find the way to collect all the keys")
