from __future__ import print_function
import fileinput
from operator import mul
from functools import reduce

totalArea = 0
totalRibbon = 0
for line in fileinput.input():
    parts = [int(i) for i in line.split('x')]
    parts.sort()

    sides = [parts[0] * parts[1], parts[0] * parts[2], parts[1] * parts[2]]
    totalArea += 2 * sum(sides)

    totalArea += min(sides)
    totalRibbon += 2 * (parts[0] + parts[1])
    totalRibbon += reduce(mul, parts, 1)

print(totalArea, "paper")
print(totalRibbon, "ribbon")

