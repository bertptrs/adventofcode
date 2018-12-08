import fileinput
import re
import itertools
from collections import defaultdict, deque
from queue import Queue, PriorityQueue, SimpleQueue


def sum_metadata(l):
    children, metaentries = l[:2]
    start = 2
    total_len = metaentries + 2

    total = 0

    for _ in range(children):
        nt, nl = sum_metadata(l[start:])
        total += nt
        total_len += nl
        start += nl

    if metaentries > 0:
        return total + sum(l[start:start+metaentries]), total_len
    else:
        return total, total_len


def sum_metadata2(l):
    children, metaentries = l[:2]
    print(children, metaentries, l)
    start = 2
    total_len = metaentries + 2

    if children == 0:
        return sum(l[start:start+metaentries]), total_len

    values = []
    for _ in range(children):
        nt, nl = sum_metadata2(l[start:])
        values.append(nt)
        total_len += nl
        start += nl

    print(values)
    print(l[start:start+metaentries])

    return sum(values[x - 1] for x in l[start:start+metaentries] if x - 1 < len(values)), total_len


numbers = [int(x) for x in next(fileinput.input()).strip().split()]
print(sum_metadata(numbers))
print(sum_metadata2(numbers))
