from collections import defaultdict

import networkx

from . import SeparateRunner


class DayRunner(SeparateRunner):
    @classmethod
    def part1(cls, input: str) -> int:
        edges = defaultdict(set)

        for line in input.strip().split("\n"):
            a, b = line.split("-")
            edges[a].add(b)
            edges[b].add(a)

        found = set()

        for a, out in edges.items():
            if a[0] != "t":
                continue

            for b in out:
                for c in edges[b]:
                    if c in out:
                        found.add(tuple(sorted([a, b, c])))

        return len(found)

    @classmethod
    def part2(cls, input: str) -> str:
        graph = networkx.Graph()

        for line in input.strip().split("\n"):
            a, b = line.split("-")
            graph.add_edge(a, b)

        cliques = networkx.find_cliques(graph)
        max_clique = max(cliques, key=len)

        return ",".join(sorted(max_clique))
