from __future__ import print_function, division
import fileinput
from collections import defaultdict

buckets = []

for line in fileinput.input():
    buckets.append(int(line))

def works(bucketCombination, target):
    for idx, value in enumerate(buckets):

        if bucketCombination % 2 == 1:
            target -= value

        bucketCombination = bucketCombination // 2

    return target == 0

def ones(x):
    n = 0
    while x > 0:
        if x % 2:
            n += 1

        x //= 2

    return n

possible = defaultdict(lambda: 0)

for x in range(1 << len(buckets)):
    if works(x, 150):
        n = ones(x)
        possible[n] += 1

print(sum(possible[x] for x in possible), possible[min(possible.keys())])
