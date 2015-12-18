from __future__ import print_function, division
import fileinput
from collections import defaultdict
import bisect

def value(buckets, choice):
    total = 0
    for value in buckets:
        if choice % 2 == 1:
            total += value

        choice //= 2

    return total

def ones(x):
    n = 0
    while x > 0:
        if x % 2:
            n += 1

        x //= 2

    return n

def partition(a_list):
    pivot = len(a_list) // 2

    return a_list[:pivot], a_list[pivot:]

def partitionList(buckets):
    result = [(value(buckets, x), ones(x)) for x in range(1 << len(buckets))]
    result.sort()
    return result

buckets = []

for line in fileinput.input():
    buckets.append(int(line))

partition1, partition2 = partition(buckets)

values1 = partitionList(partition1)
values2 = partitionList(partition2)

possible = defaultdict(lambda: 0)

i = 0

target = 150

for entry in values1:

    i = bisect.bisect_left(values2, (target - entry[0], 0))

    while i < len(values2) and entry[0] + values2[i][0] == target:
        possible[entry[1] + values2[i][1]] += 1
        i += 1

print("Total possibilities:", sum(possible.values()))
print("Minimal possibilities:", possible[min(possible.keys())])

