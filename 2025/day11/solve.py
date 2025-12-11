#!/usr/bin/env python3

from collections import defaultdict
import fileinput
from typing import Iterable

import networkx as nx

def parse_graph() -> nx.Graph:
    graph = nx.DiGraph()

    for line in fileinput.input():
        source, rem = line.split(": ")
        
        for sink in rem.strip().split(" "):
            graph.add_edge(source, sink)

    return graph

def count(gen: Iterable) -> int:
    return sum(1 for _ in gen)

def main() -> None:
    graph = parse_graph()

    # Observation: graph is a DAG, so one needs to go in front of the other. We can do this in three
    # steps:
    # svr → closest(dac, fft) -> furthest(dac,fft)
    rank = {
        node: rank
        for rank, node in enumerate(nx.topological_sort(graph))
    }

    rev_rank = {rank: node for node, rank in rank.items()}

    if rank["dac"] > rank["fft"]:
        closest = "fft"
        furthest = "dac"
    else:
        closest = "dac"
        furthest = "fft"

    
    def ranked_all_paths(source: str, dest: str) -> int:
        counts = defaultdict(int)

        counts[dest] = 1

        for r in range(rank[dest], rank[source], -1):
            node = rev_rank[r]
            if node not in counts:
                continue

            for u, _ in graph.in_edges(node):
                counts[u] += counts[node]

        return counts[source]

    assert nx.has_path(graph, closest, furthest)

    print("Part 1", ranked_all_paths("you", "out"))

    first = ranked_all_paths("svr", closest)
    second = ranked_all_paths(closest, furthest)
    third = ranked_all_paths(furthest, "out")
    print("Part 2:", first * second * third)


if __name__ == "__main__":
    main()
