import collections
import heapq
from typing import TextIO


def find_middle(map_: list[str]) -> tuple[int, int, int, int]:
    mx = len(map_[0]) // 2
    my = len(map_) // 2

    maze_parts = [".", "#"]

    tx = mx
    ty = my
    bx = mx
    by = my

    while map_[ty - 1][mx] not in maze_parts:
        ty -= 1

    while map_[my][tx - 1] not in maze_parts:
        tx -= 1

    while map_[by + 1][mx] not in maze_parts:
        by += 1

    while map_[my][bx + 1] not in maze_parts:
        bx += 1

    return tx, ty, bx, by


def scan_row(
    map_: list[str], row_id: int, top: bool, outer: bool
) -> dict[tuple[int, int], tuple[str, bool, tuple[int, int]]]:
    result = {}

    position = row_id + 1 if top else row_id
    entrance = row_id + 2 if top else row_id - 1

    for x, (c, d) in enumerate(zip(map_[row_id], map_[row_id + 1])):
        if c.isupper() and d.isupper():
            result[(x, position)] = (c + d, outer, (x, entrance))

    return result


def scan_column(
    map_: list[str],
    col_id: int,
    left: bool,
    outer: bool,
) -> dict[tuple[int, int], tuple[str, bool, tuple[int, int]]]:
    position = col_id + 1 if left else col_id
    entrance = col_id + 2 if left else col_id - 1

    result = {}

    for y, row in enumerate(map_):
        if not row.strip():
            continue

        name = row[col_id : col_id + 2]
        if all(c.isupper() for c in name):
            result[(position, y)] = (name, outer, (entrance, y))

    return result


def build_map(
    map_: list[str], portals: dict[tuple[int, int], tuple[str, bool, tuple[int, int]]]
) -> dict[tuple[str, bool], list[tuple[tuple[str, bool], int]]]:
    portal_index = {
        (name, outer): (entrance, []) for name, outer, entrance in portals.values()
    }

    for (name, outer), (entrance, neighbours) in portal_index.items():
        todo = collections.deque()
        visited = {entrance}
        todo.append((entrance, 0))

        while todo:
            (x, y), dist = todo.popleft()

            for dx, dy in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
                nx = x + dx
                ny = y + dy

                if (nx, ny) in visited:
                    continue

                visited.add((nx, ny))

                match map_[ny][nx]:
                    case "#":
                        continue
                    case ".":
                        todo.append(((nx, ny), dist + 1))
                    case _ if (nx, ny) in portals:
                        dest, douter, _ = portals[nx, ny]
                        if dest == "AA" or dest == name and outer == douter:
                            # Self loop
                            continue
                        neighbours.append(((dest, douter), dist + 1))

    return {portal: neighbours for portal, (_, neighbours) in portal_index.items()}


def find_portals(
    map_: list[str],
) -> dict[tuple[int, int], tuple[str, bool, tuple[int, int]]]:
    width = len(map_[0])
    height = len(map_) - 1  # N.B. empty line at the end

    tx, ty, bx, by = find_middle(map_)

    portals = scan_row(map_, 0, outer=True, top=True)
    portals.update(scan_row(map_, ty, outer=False, top=False))
    portals.update(scan_row(map_, by - 1, outer=False, top=True))
    portals.update(scan_row(map_, height - 1, outer=True, top=False))

    portals.update(scan_column(map_, 0, outer=True, left=True))
    portals.update(scan_column(map_, tx, outer=False, left=False))
    portals.update(scan_column(map_, bx - 1, outer=False, left=True))
    portals.update(scan_column(map_, width - 2, outer=True, left=False))

    return portals


def part1(data: TextIO) -> int:
    # Can't use strip() because the first line starts with significant whitespace
    map_ = data.read().splitlines()

    portals = find_portals(map_)

    portal_index = build_map(map_, portals)

    best = {
        ("AA", True): 0,
    }

    todo = [(0, ("AA", True))]

    while todo:
        dist, state = heapq.heappop(todo)

        if best[state] < dist:
            continue

        pos, _ = state
        if pos == "ZZ":
            return dist - 1

        for neighbour, delta in portal_index[state]:
            new_dist = dist + delta
            name, outer = neighbour
            dest = (name, not outer)

            if dest not in best or best[dest] > new_dist:
                best[dest] = new_dist
                heapq.heappush(todo, (new_dist, dest))

    raise ValueError("Did not find a way out")


def part2(data: TextIO) -> int:
    # Can't use strip() because the first line starts with significant whitespace
    map_ = data.read().splitlines()

    portals = find_portals(map_)

    portal_index = build_map(map_, portals)

    best = {
        (0, "AA", True): 0,
    }

    todo = [(0, (0, "AA", True))]

    while todo:
        dist, state = heapq.heappop(todo)

        if best[state] < dist:
            continue

        level, pos, outer = state
        if pos == "ZZ":
            return dist - 1

        for neighbour, delta in portal_index[(pos, outer)]:
            new_dist = dist + delta
            name, outer = neighbour

            if name == "ZZ" and level != 0:
                # Is a wall
                continue

            if name != "ZZ" and outer and level <= 0:
                continue

            next_level = level - 1 if outer else level + 1

            dest = (next_level, name, not outer)

            if dest not in best or best[dest] > new_dist:
                best[dest] = new_dist
                heapq.heappush(todo, (new_dist, dest))

    raise ValueError("Did not find a way out")
