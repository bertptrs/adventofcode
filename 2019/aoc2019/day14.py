import math
from collections import defaultdict
from typing import TextIO, Tuple

from networkx import DiGraph, topological_sort  # type: ignore[import]


def read_pair(item: str) -> Tuple[str, int]:
    amount, element = item.split(' ')

    return element, int(amount)


def read_recipes(data: TextIO) -> DiGraph:
    graph = DiGraph()

    for line in data:
        requisites, production = line.strip().split(' => ')

        produced, produced_amount = read_pair(production)
        graph.add_node(produced, weight=produced_amount)

        for requisite in requisites.split(', '):
            required, required_amount = read_pair(requisite)
            graph.add_edge(produced, required, weight=required_amount)

    return graph


def ore_required(graph: DiGraph, fuel_required: int) -> int:
    requirements = defaultdict(int)
    requirements['FUEL'] = fuel_required

    for element in topological_sort(graph):
        if element not in requirements:
            continue

        if element == 'ORE':
            break

        element_produced = graph.nodes[element]['weight']
        productions_required = math.ceil(requirements[element] / element_produced)

        for _, elem_required, amount_required in graph.edges(element, data='weight'):
            requirements[elem_required] += amount_required * productions_required

    return requirements['ORE']


def part1(data: TextIO) -> int:
    return ore_required(read_recipes(data), 1)


def part2(data: TextIO) -> int:
    ore_available = 1000000000000
    graph = read_recipes(data)

    min_possible = 1  # lower bound of ORE / ore_required(graph, 1) exists but is slower
    max_possible = ore_available

    while min_possible != max_possible:
        check = min_possible + (max_possible - min_possible + 1) // 2

        required = ore_required(graph, check)

        if required <= ore_available:
            min_possible = check
        else:
            max_possible = check - 1

    return min_possible
