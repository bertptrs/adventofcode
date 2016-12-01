from __future__ import print_function
import itertools

def lookandsay(line):
    return ''.join([str(len(list(it))) + c for c, it in itertools.groupby(line)])


line = "1321131112"
for x in range(40):
    line = lookandsay(line)

print("40:", len(line))

for x in range(10):
    line = lookandsay(line)

print("50:", len(line))
