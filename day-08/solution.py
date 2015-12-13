import fileinput
import re

totalLess = 0
totalMore = 0

for line in fileinput.input():
    original = len(line)
    diff = 2
    plus = 2
    escaped = False
    for c in line:
        if c == "\\" or c == "\"":
            plus += 1

        if c == "\\" and not escaped:
            escaped = True
            diff += 1
            continue

        if escaped and c == "x":
            diff += 2

        escaped = False

    totalLess += diff
    totalMore += plus

print(totalLess, "less")
print(totalMore, "more")
