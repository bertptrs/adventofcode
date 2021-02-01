import sys
from collections import defaultdict
from typing import TextIO

import networkx  # type: ignore

from aoc2019.intcode import Computer, read_program


def step(computer: Computer, direction: int) -> int:
    computer.input.append(direction)

    try:
        computer.run()
    except IndexError:
        return computer.get_output()

    sys.exit("computer terminated unexpectedly")


def inverse(direction: int):
    if direction % 2 == 1:
        return direction + 1
    else:
        return direction - 1


def read_graph(data: TextIO) -> networkx.Graph:
    computer = Computer(read_program(data))

    pos = (0, 0)
    tiles = defaultdict(int)
    tiles[0, 0] = 1

    prev = [((0, 0), 1)]

    while prev:
        x, y = pos

        if (x, y + 1) not in tiles:
            movement = 1
            next_pos = (x, y + 1)
        elif (x, y - 1) not in tiles:
            movement = 2
            next_pos = (x, y - 1)
        elif (x - 1, y) not in tiles:
            movement = 3
            next_pos = (x - 1, y)
        elif (x + 1, y) not in tiles:
            movement = 4
            next_pos = (x + 1, y)
        else:
            # No movement available, backtrack
            prev_pos, prev_dir = prev.pop()
            step(computer, inverse(prev_dir))
            pos = prev_pos
            continue

        result = step(computer, movement)
        tiles[next_pos] = result

        if result != 0:
            # Movement was successful
            prev.append((pos, movement))
            pos = next_pos

    graph = networkx.Graph()

    for pos, value in tiles.items():
        if value == 0:
            continue

        if value == 2:
            # Create an imaginary edge to the oxygen
            graph.add_edge('O', pos, weight=0)

        x, y = pos

        neighbours = [
            (x - 1, y),
            (x + 1, y),
            (x, y - 1),
            (x, y + 1),
        ]

        for neighbour in neighbours:
            if tiles[neighbour] != 0:
                graph.add_edge(pos, neighbour)

    return graph


def part1(data: TextIO) -> int:
    graph = read_graph(data)

    return networkx.shortest_path_length(graph, (0, 0), 'O') - 1


def part2(data: TextIO) -> int:
    graph = read_graph(data)

    return networkx.eccentricity(graph, 'O') - 1
