from typing import TextIO

import networkx


def read_graph(data: TextIO) -> networkx.DiGraph:
    graph = networkx.DiGraph()

    for line in data:
        a, b = line.strip().split(')')
        graph.add_edge(a, b)

    return graph


def part1(data: TextIO) -> int:
    graph = read_graph(data)

    paths = networkx.single_source_shortest_path_length(graph, 'COM')

    return sum(paths.values())


def part2(data: TextIO) -> int:
    graph = read_graph(data).to_undirected()

    return networkx.shortest_path_length(graph, 'YOU', 'SAN') - 2
