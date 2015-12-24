from __future__ import print_function, division
import fileinput
import itertools
import functools
import operator

def qes(packageList):
    return functools.reduce(operator.mul, packageList)

def minQES(packages, slots):
    targetWeight = sum(packages) // slots
    for i in range(1, len(packages)):
        solutions = [x for x in itertools.combinations(packages, i) if sum(x) == targetWeight]
        if len(solutions) > 0:
            return min(qes(x) for x in solutions)

packages = set([int(x) for x in fileinput.input()])

print(minQES(packages, 3))
print(minQES(packages, 4))
